use {
    crate::{
        constants::REST_BASE_URL,
        models::{
            error_response::ErrorResponse,
            instruments::instruments_response::InstrumentsResponse,
            success_response::SuccessResponse,
            user::profile_response::ProfileResponse,
            ws::{
                portfolio_feed_request::PortfolioUpdateType,
                portfolio_feed_response::PortfolioFeedResponse,
            },
            ExchangeSegment,
        },
        protos::market_data_feed::FeedResponse as MarketDataFeedResponse,
        ws_client::{MarketDataFeedClient, PortfolioFeedClient},
    },
    chrono::FixedOffset,
    ezsockets::Client as EzClient,
    reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response},
    serde::Serialize,
    std::{
        collections::{HashMap, HashSet},
        sync::Arc,
    },
    tokio::{
        sync::{Mutex, MutexGuard},
        task::JoinHandle,
    },
    tokio_cron_scheduler::{Job, JobScheduler},
};

#[derive(Debug)]
pub struct ApiClient<F, G>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
    G: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    pub(crate) client: ReqwestClient,
    pub(crate) api_key: String,
    pub(crate) token: Option<String>,
    pub instruments: Option<HashMap<ExchangeSegment, HashMap<String, Vec<InstrumentsResponse>>>>,
    pub portfolio_feed_client: Option<EzClient<PortfolioFeedClient<F>>>,
    pub market_data_feed_client: Option<EzClient<MarketDataFeedClient<G>>>,
}

impl<F, G> ApiClient<F, G>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
    G: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    pub async fn new(
        api_key: &str,
        login_config: LoginConfig,
        fetch_instruments: bool,
        schedule_refresh_instruments: bool,
        ws_connect_config: WSConnectConfig<F, G>,
    ) -> Result<(Arc<Mutex<ApiClient<F, G>>>, Vec<JoinHandle<()>>), String> {
        // TODO replace print with tracing
        let api_client: ApiClient<F, G> = ApiClient {
            client: ReqwestClient::new(),
            api_key: api_key.to_string(),
            token: None,
            instruments: None,
            portfolio_feed_client: None,
            market_data_feed_client: None,
        };

        let shared_api_client: Arc<Mutex<ApiClient<F, G>>> = Arc::new(Mutex::new(api_client));
        let mut tasks_vec: Vec<JoinHandle<()>> = Vec::<JoinHandle<()>>::new();

        let scheduler: JobScheduler = JobScheduler::new().await.unwrap();
        scheduler.start().await.unwrap();
        scheduler.shutdown_on_ctrl_c();

        if fetch_instruments {
            let mut api_client: MutexGuard<ApiClient<F, G>> = shared_api_client.lock().await;
            api_client.instruments =
                Some(Self::parse_instruments(api_client.get_instruments().await?));
            if schedule_refresh_instruments {
                Self::schedule_refresh_instruments(&scheduler, &shared_api_client).await;
            }
        }

        if !login_config.authorize {
            return Ok((shared_api_client, tasks_vec));
        }

        {
            let mut api_client: MutexGuard<ApiClient<F, G>> = shared_api_client.lock().await;
            api_client.login(&login_config).await?;

            if ws_connect_config.connect_portfolio_stream {
                let portfolio_feed_task: JoinHandle<()> = api_client
                    .connect_portfolio_feed(
                        ws_connect_config.portfolio_stream_update_types,
                        ws_connect_config.portfolio_feed_callback,
                    )
                    .await?;
                tasks_vec.push(portfolio_feed_task);
            }
            if ws_connect_config.connect_market_data_stream {
                let market_data_feed_task: JoinHandle<()> = api_client
                    .connect_market_data_feed(ws_connect_config.market_data_feed_callback)
                    .await?;
                tasks_vec.push(market_data_feed_task);
            }
        }

        if let Some(automate_login_config) = login_config.automate_login_config {
            if automate_login_config.schedule_login {
                Self::schedule_auto_login(&scheduler, &shared_api_client, login_config).await;
            }
        }
        Ok((shared_api_client, tasks_vec))
    }

    pub async fn get(
        &self,
        endpoint: &str,
        authorized: bool,
        params: Option<&Vec<(String, String)>>,
    ) -> Response {
        self.request::<()>(Method::GET, endpoint, authorized, params, None, None)
            .await
    }

    pub async fn post<T>(
        &self,
        endpoint: &str,
        authorized: bool,
        json_body: Option<&T>,
        form_body: Option<&Vec<(String, String)>>,
    ) -> Response
    where
        T: Serialize + ?Sized,
    {
        self.request(
            Method::POST,
            endpoint,
            authorized,
            None,
            json_body,
            form_body,
        )
        .await
    }

    pub async fn put<T>(
        &self,
        endpoint: &str,
        authorized: bool,
        json_body: Option<&T>,
        form_body: Option<&Vec<(String, String)>>,
    ) -> Response
    where
        T: Serialize + ?Sized,
    {
        self.request(
            Method::PUT,
            endpoint,
            authorized,
            None,
            json_body,
            form_body,
        )
        .await
    }

    pub async fn delete(
        &self,
        endpoint: &str,
        authorized: bool,
        params: Option<&Vec<(String, String)>>,
    ) -> Response {
        self.request::<()>(Method::DELETE, endpoint, authorized, params, None, None)
            .await
    }

    async fn request<T>(
        &self,
        method: Method,
        endpoint: &str,
        authorized: bool,
        params: Option<&Vec<(String, String)>>,
        json_body: Option<&T>,
        form_body: Option<&Vec<(String, String)>>,
    ) -> Response
    where
        T: Serialize + ?Sized,
    {
        let url: String = format!("{}{}", REST_BASE_URL, endpoint);

        if authorized && !self.token.is_some() {
            panic!(
                "{}",
                format!(
                    "Cannot make authorized requests as client is not authorized: {}",
                    url
                )
            );
        }

        let mut request: RequestBuilder = match method {
            Method::GET => self.client.get(url),
            Method::POST => self.client.post(url),
            Method::PUT => self.client.put(url),
            Method::DELETE => self.client.delete(url),
            _ => panic!("Unsupported HTTP Method"),
        };

        if let Some(req_params) = params {
            request = request.query(req_params);
        }

        if let Some(req_json_body) = json_body {
            request = request.json(req_json_body);
        }

        if let Some(req_form_body) = form_body {
            request = request.form(req_form_body);
        }

        if authorized {
            request = request.bearer_auth(&self.token.as_ref().unwrap());
        }
        request = request.header("Accept", "application/json");
        request.send().await.unwrap()
    }

    pub(crate) async fn verify_authorization(&mut self) -> bool {
        let verify_response: Result<SuccessResponse<ProfileResponse>, ErrorResponse> =
            self.get_profile().await;
        verify_response.map_or_else(
            |_| {
                println!("Access Token invalid");
                false
            },
            |_| {
                println!("Using valid access token from file");
                true
            },
        )
    }

    async fn schedule_refresh_instruments(
        scheduler: &JobScheduler,
        shared_api_client: &Arc<Mutex<ApiClient<F, G>>>,
    ) {
        let shared_api_client_clone: Arc<Mutex<ApiClient<F, G>>> = Arc::clone(&shared_api_client);
        let job: Job = Job::new_async_tz(
            "0 30 06 * * *",
            FixedOffset::east_opt(19800).unwrap(),
            move |_, _| {
                let api_client: Arc<Mutex<ApiClient<F, G>>> = Arc::clone(&shared_api_client_clone);
                Box::pin(async move {
                    let mut client: MutexGuard<ApiClient<F, G>> = api_client.lock().await;
                    if let Ok(instruments) = client.get_instruments().await {
                        client.instruments = Some(Self::parse_instruments(instruments));
                    }
                })
            },
        )
        .unwrap();

        scheduler.add(job).await.unwrap();
    }

    async fn schedule_auto_login(
        scheduler: &JobScheduler,
        shared_api_client: &Arc<Mutex<ApiClient<F, G>>>,
        login_config: LoginConfig,
    ) {
        let shared_api_client_clone: Arc<Mutex<ApiClient<F, G>>> = Arc::clone(&shared_api_client);
        let job: Job = Job::new_async_tz(
            "0 30 03 * * *",
            FixedOffset::east_opt(19800).unwrap(),
            move |_, _| {
                let api_client: Arc<Mutex<ApiClient<F, G>>> = Arc::clone(&shared_api_client_clone);
                let login_config: LoginConfig = login_config.clone();
                Box::pin(async move {
                    let mut client: MutexGuard<ApiClient<F, G>> = api_client.lock().await;
                    client.login(&login_config).await.unwrap();
                })
            },
        )
        .unwrap();

        scheduler.add(job).await.unwrap();
    }
}

#[derive(Clone)]
pub struct LoginConfig {
    pub authorize: bool,
    pub automate_login_config: Option<AutomateLoginConfig>,
}

#[derive(Clone, Copy)]
pub struct AutomateLoginConfig {
    pub automate_login: bool,
    pub schedule_login: bool, // At 3:30 AM IST daily
    pub automate_fetching_otp: bool,
    pub mail_provider: Option<MailProvider>,
}

#[derive(Clone, Copy)]
pub enum MailProvider {
    Google,
}

pub struct WSConnectConfig<F, G>
where
    F: FnMut(PortfolioFeedResponse) + Send + Sync + 'static,
    G: FnMut(MarketDataFeedResponse) + Send + Sync + 'static,
{
    pub connect_portfolio_stream: bool,
    pub connect_market_data_stream: bool,
    pub portfolio_stream_update_types: Option<HashSet<PortfolioUpdateType>>,
    pub portfolio_feed_callback: Option<F>,
    pub market_data_feed_callback: Option<G>,
}
