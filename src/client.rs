//! ## Get started
//!
//! The code below represents an API client that connects websockets and uses handlers to handle incoming data.
//!
//!
//! ```rust
//! use {
//!     dotenvy::dotenv,
//!     futures::future::join_all,
//!     std::{collections::HashSet, env},
//!     tokio::signal,
//!     upstox_rust_sdk::{
//!         client::{ApiClient, AutomateLoginConfig, LoginConfig, MailProvider, WSConnectConfig},
//!         constants::UPLINK_API_KEY_ENV,
//!         models::ws::{
//!             market_data_feed_v3_message::{MessageDataV3, ModeTypeV3},
//!             portfolio_feed_request::PortfolioUpdateType,
//!             portfolio_feed_response::PortfolioFeedResponse,
//!         },
//!         protos::market_data_feed_v3::FeedResponse as MarketDataFeedV3Response,
//!         ws_client::MarketDataV3Call,
//!     },
//! };

//! #[tokio::main]
//! async fn main() {
//!     tracing_subscriber::fmt::init();
//!     let _ = dotenv();

//!     let portfolio_feed_handler = |data: PortfolioFeedResponse| {
//!         println!("{:?}", data);
//!     };
//!     let market_data_feed_v3_handler = |data: MarketDataFeedV3Response| {
//!         println!("{:?}", data);
//!     };

//!     let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
//!     let (fetch_instruments, schedule_refresh_instruments) = (false, false);

//!     // ApiClient with websockets connected and handler specified
//!     let (api_client, tasks_vec) = ApiClient::new(
//!         &api_key,
//!         LoginConfig {
//!             authorize: true,
//!             automate_login_config: Some(AutomateLoginConfig {
//!                 automate_login: true,
//!                 schedule_login: false,
//!                 automate_fetching_otp: true,
//!                 mail_provider: Some(MailProvider::Google),
//!             }),
//!         },
//!         fetch_instruments,
//!         schedule_refresh_instruments,
//!         // Configuration to connect and handle websocket data.
//!         WSConnectConfig {
//!             connect_portfolio_stream: true,
//!             connect_market_data_stream_v3: true,
//!             // Select which portfolio data to fetch
//!             portfolio_stream_update_types: Some(HashSet::from([
//!                 PortfolioUpdateType::Order,
//!                 PortfolioUpdateType::Position,
//!                 PortfolioUpdateType::Holding,
//!             ])),
//!             // Handle portfolio data feed
//!             portfolio_feed_callback: Some(Box::new(portfolio_feed_handler)),
//!             // Handle market data feed
//!             market_data_feed_v3_callback: Some(Box::new(market_data_feed_v3_handler)),
//!         },
//!     )
//!     .await
//!     .unwrap();

//!     let api_client = api_client.lock().await;
//!     api_client
//!         .send_market_data_feed_v3_message(MarketDataV3Call::SubscribeInstrument(MessageDataV3 {
//!             mode: ModeTypeV3::Full,
//!             instrument_keys: vec![
//!                 "NSE_INDEX|NIFTY LARGEMID250".to_string(),
//!                 "NSE_INDEX|Nifty Auto".to_string(),
//!                 "NSE_INDEX|Nifty Midcap 50".to_string(),
//!             ],
//!         }))
//!         .await
//!         .unwrap();

//!     // This ensures that app continues running until the websockets die if connected or until SIGINT occurs
//!     tokio::select! {
//!         _ = join_all(tasks_vec) => {}
//!         _ = signal::ctrl_c() => {}
//!     };
//! }

//!
//! ```
//!
//! The code below represents an API client that logs user in, allowing usage of authorised endpoints, automates the login process including fetching OTP via GMail and scheduling it daily.
//!
//!
//! ```rust
//!
//! #[tokio::main]
//! async fn main() {
//!     tracing_subscriber::fmt::init();
//!     let _ = dotenv();

//!     let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
//!     let (fetch_instruments, schedule_refresh_instruments) = (false, false);

//!     // ApiClient which logs in automatically and schedules relogin daily when token expires
//!     let (_api_client, tasks_vec) = ApiClient::new(
//!         &api_key,
//!         LoginConfig {
//!             authorize: true,
//!             automate_login_config: Some(AutomateLoginConfig {
//!                 // geckodriver or chromedriver binary must be running locally with port specified in env to use automatic login or schedule login.
//!                 // ./geckodriver --binary "~/.local/share/flatpak/exports/bin/org.mozilla.firefox" --profile-root "~/.var/app/org.mozilla.firefox/cache/mozilla/firefox/cv70hco5.default-release"

//!                 // Either GOOGLE_AUTHORIZATION_CODE environment must not be set or must be recent if using for the first time or refresh_token.txt has been deleted
//!                 automate_login: true,
//!                 // Relogin is scheduled at 3:30 AM IST daily.
//!                 schedule_login: true,
//!                 // Fetch OTP automatically from inbox by providing email IMAP access using env variables. For GMail, login page will be opened in your default web browsers and permission must be granted.
//!                 automate_fetching_otp: true,
//!                 mail_provider: Some(MailProvider::Google),
//!             }),
//!         },
//!         fetch_instruments,
//!         schedule_refresh_instruments,
//!         WSConnectConfig {
//!             connect_portfolio_stream: false,
//!             connect_market_data_stream_v3: false,
//!             portfolio_stream_update_types: None,
//!             portfolio_feed_callback: None::<Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>>,
//!             market_data_feed_v3_callback: None::<
//!                 Box<dyn FnMut(MarketDataFeedV3Response) + Send + Sync>,
//!             >,
//!         },
//!     )
//!     .await
//!     .unwrap();

//!     // This ensures that app continues running until SIGINT occurs
//!     tokio::select! {
//!         _ = join_all(tasks_vec) => {}
//!         _ = signal::ctrl_c() => {}
//!     };
//! }
//!
//!
//! ```
//!
//! The code below represents an API client that fetches the complete list of instruments and stores it in the API Client. It also schedules daily refresh of the list.
//!
//! ```rust
//! #[tokio::main]
//! async fn main() {
//!     tracing_subscriber::fmt::init();
//!     let _ = dotenv();

//!     let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
//!     let (fetch_instruments, schedule_refresh_instruments) = (true, true);

//!     // ApiClient which fetches instruments, schedules instruments refresh daily and stores it in ApiClient
//!     let (api_client, tasks_vec) = ApiClient::new(
//!         &api_key,
//!         LoginConfig {
//!             authorize: false,
//!             automate_login_config: Some(AutomateLoginConfig {
//!                 automate_login: false,
//!                 schedule_login: false,
//!                 automate_fetching_otp: false,
//!                 mail_provider: Some(MailProvider::Google),
//!             }),
//!         },
//!         // Fetch all instruments data from UPSTOX and store it in the ApiClient.
//!         fetch_instruments,
//!         // Refresh instruments data daily at 6:30 AM.
//!         schedule_refresh_instruments,
//!         WSConnectConfig {
//!             connect_portfolio_stream: false,
//!             connect_market_data_stream_v3: false,
//!             portfolio_stream_update_types: None,
//!             portfolio_feed_callback: None::<Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>>,
//!             market_data_feed_v3_callback: None::<
//!                 Box<dyn FnMut(MarketDataFeedV3Response) + Send + Sync>,
//!             >,
//!         },
//!     )
//!     .await
//!     .unwrap();

//!     {
//!         // Ensure that api_client mutex guard goes out of scope when no longer needed.
//!         let api_client: MutexGuard<ApiClient> = api_client.lock().await;
//!         print!(
//!             "{:?}",
//!             api_client
//!                 .instruments
//!                 .as_ref()
//!                 .unwrap()
//!                 .get(&ExchangeSegment::NseIndex)
//!                 .unwrap()
//!                 .get("INDEX")
//!                 .unwrap()
//!         );
//!         std::io::stdout().flush().unwrap();
//!     }

//!     // This ensures that app continues running until SIGINT occurs
//!     tokio::select! {
//!         _ = join_all(tasks_vec) => {}
//!         _ = signal::ctrl_c() => {}
//!     };
//! }
//! ```
//!
//! The code below represents the basic usage of API Client to fetch data via REST APIs.
//!
//! ```rust
//! use {
//!     dotenvy::dotenv,
//!     futures::future::join_all,
//!     std::env,
//!     tokio::{
//!         signal,
//!         sync::MutexGuard,
//!         time::{Instant, sleep},
//!     },
//!     tracing::info,
//!     upstox_rust_sdk::{
//!         client::{ApiClient, AutomateLoginConfig, LoginConfig, MailProvider, WSConnectConfig},
//!         constants::UPLINK_API_KEY_ENV,
//!         models::{
//!             ProductType, TransactionType,
//!             charges::brokerage_details_request::BrokerageDetailsRequest,
//!             success_response::SuccessResponse,
//!             user::{
//!                 fund_and_margin_request::SegmentType,
//!                 fund_and_margin_response::FundAndMarginResponse, profile_response::ProfileResponse,
//!             },
//!             ws::portfolio_feed_response::PortfolioFeedResponse,
//!         },
//!         protos::market_data_feed_v3::FeedResponse as MarketDataFeedV3Response,
//!         rate_limiter::RateLimitExceeded,
//!     },
//! };

//! #[tokio::main]
//! async fn main() {
//!     tracing_subscriber::fmt::init();
//!     let _ = dotenv();

//!     let api_key: String = env::var(UPLINK_API_KEY_ENV).unwrap();
//!     let (fetch_instruments, schedule_refresh_instruments) = (true, false);

//!     let (api_client, tasks_vec) = ApiClient::new(
//!         &api_key,
//!         LoginConfig {
//!             authorize: true,
//!             automate_login_config: Some(AutomateLoginConfig {
//!                 automate_login: true,
//!                 schedule_login: false,
//!                 automate_fetching_otp: true,
//!                 mail_provider: Some(MailProvider::Google),
//!             }),
//!         },
//!         fetch_instruments,
//!         schedule_refresh_instruments,
//!         WSConnectConfig {
//!             connect_portfolio_stream: false,
//!             connect_market_data_stream_v3: false,
//!             portfolio_stream_update_types: None,
//!             portfolio_feed_callback: None::<Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>>,
//!             market_data_feed_v3_callback: None::<
//!                 Box<dyn FnMut(MarketDataFeedV3Response) + Send + Sync>,
//!             >,
//!         },
//!     )
//!     .await
//!     .unwrap();

//!     {
//!         let api_client: MutexGuard<ApiClient> = api_client.lock().await;

//!         // User Endpoints
//!         let _profile: SuccessResponse<ProfileResponse> =
//!             api_client.get_profile().await.unwrap().unwrap();
//!         info!("Profile: {:?}", _profile);

//!         // Charges Endpoints
//!         let _charges_result = api_client
//!             .get_brokerage_details(BrokerageDetailsRequest {
//!                 instrument_token: "NSE_EQ|INE806T01012".to_string(),
//!                 quantity: 2,
//!                 product: ProductType::I,
//!                 transaction_type: TransactionType::Buy,
//!                 price: 1575.00,
//!             })
//!             .await;

//!         if let Err(e) = _charges_result {
//!             let next_allowed_at = match e {
//!                 RateLimitExceeded::PerSecond { next_allowed_at } => next_allowed_at,
//!                 RateLimitExceeded::PerMinute { next_allowed_at } => next_allowed_at,
//!                 RateLimitExceeded::PerThirtyMinutes { next_allowed_at } => next_allowed_at,
//!             };
//!             info!(
//!                 "Rate limit exceeded. Sleeping for {:?}",
//!                 next_allowed_at - Instant::now()
//!             );
//!             sleep(next_allowed_at - Instant::now()).await;
//!         }

//!         let charges = api_client
//!             .get_brokerage_details(BrokerageDetailsRequest {
//!                 instrument_token: "NSE_EQ|INE806T01012".to_string(),
//!                 quantity: 2,
//!                 product: ProductType::I,
//!                 transaction_type: TransactionType::Buy,
//!                 price: 1575.00,
//!             })
//!             .await
//!             .unwrap()
//!             .unwrap();

//!         info!("Charges: {:?}", charges);

//!         let _funds_and_margin: SuccessResponse<FundAndMarginResponse> = api_client
//!             .get_fund_and_margin(Some(SegmentType::Sec))
//!             .await
//!             .unwrap()
//!             .unwrap();

//!         // This is just for usage illustration. All endpoints in https://upstox.com/developer/api-documentation/open-api are available via the ApiClient.
//!     }

//!     // This ensures that app continues running until SIGINT occurs
//!     tokio::select! {
//!         _ = join_all(tasks_vec) => {}
//!         _ = signal::ctrl_c() => {}
//!     };
//! }
//!
//! ```

use {
    crate::{
        constants::{
            APIVersion, BaseUrlType, RATE_LIMIT_PER_MINUTE, RATE_LIMIT_PER_SECOND,
            RATE_LIMIT_PER_THIRTY_MINUTES,
        },
        models::{
            ExchangeSegment,
            error_response::ErrorResponse,
            instruments::instruments_response::InstrumentsResponse,
            success_response::SuccessResponse,
            user::profile_response::ProfileResponse,
            ws::{
                portfolio_feed_request::PortfolioUpdateType,
                portfolio_feed_response::PortfolioFeedResponse,
            },
        },
        protos::market_data_feed_v3::FeedResponse as MarketDataFeedV3Response,
        rate_limiter::{ApiRateLimiter, RateLimitExceeded},
        utils::create_url,
        ws_client::{MarketDataFeedV3Client, PortfolioFeedClient},
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
    tracing::info,
};

pub struct ApiClient {
    pub(crate) client: ReqwestClient,
    pub(crate) api_key: String,
    pub(crate) token: Option<String>,
    pub instruments: Option<HashMap<ExchangeSegment, HashMap<String, Vec<InstrumentsResponse>>>>,
    pub portfolio_feed_client:
        Option<EzClient<PortfolioFeedClient<Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>>>>,
    pub market_data_feed_v3_client: Option<
        EzClient<MarketDataFeedV3Client<Box<dyn FnMut(MarketDataFeedV3Response) + Send + Sync>>>,
    >,
    pub rate_limiter: ApiRateLimiter,
}

impl ApiClient {
    pub async fn new(
        api_key: &str,
        login_config: LoginConfig,
        fetch_instruments: bool,
        schedule_refresh_instruments: bool,
        ws_connect_config: WSConnectConfig,
    ) -> Result<(Arc<Mutex<ApiClient>>, Vec<JoinHandle<()>>), String> {
        let api_client = ApiClient {
            client: ReqwestClient::new(),
            api_key: api_key.to_string(),
            token: None,
            instruments: None,
            portfolio_feed_client: None,
            market_data_feed_v3_client: None,
            rate_limiter: ApiRateLimiter::new(
                RATE_LIMIT_PER_SECOND,
                RATE_LIMIT_PER_MINUTE,
                RATE_LIMIT_PER_THIRTY_MINUTES,
            ),
        };

        let shared_api_client = Arc::new(Mutex::new(api_client));
        let mut tasks_vec = Vec::<JoinHandle<()>>::new();

        let scheduler = JobScheduler::new().await.unwrap();
        scheduler.start().await.unwrap();
        scheduler.shutdown_on_ctrl_c();

        if fetch_instruments {
            let mut api_client = shared_api_client.lock().await;
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
            let mut api_client = shared_api_client.lock().await;
            api_client.login(&login_config).await?;

            if ws_connect_config.connect_portfolio_stream {
                let portfolio_feed_task = api_client
                    .connect_portfolio_feed(
                        ws_connect_config.portfolio_stream_update_types,
                        ws_connect_config.portfolio_feed_callback,
                    )
                    .await?;
                tasks_vec.push(portfolio_feed_task);
            }
            if ws_connect_config.connect_market_data_stream_v3 {
                let market_data_feed_v3_task = api_client
                    .connect_market_data_feed_v3(ws_connect_config.market_data_feed_v3_callback)
                    .await?;
                tasks_vec.push(market_data_feed_v3_task);
            }
        }

        if let Some(automate_login_config) = login_config.automate_login_config {
            if automate_login_config.schedule_login {
                Self::schedule_auto_login(&scheduler, &shared_api_client, login_config).await;
            }
        }
        Ok((shared_api_client, tasks_vec))
    }

    pub(crate) async fn get(
        &self,
        endpoint: &str,
        authorized: bool,
        params: Option<&Vec<(String, String)>>,
        base_url_type: BaseUrlType,
        api_version: APIVersion,
    ) -> Result<Response, RateLimitExceeded> {
        self.request::<()>(
            Method::GET,
            endpoint,
            authorized,
            params,
            None,
            None,
            base_url_type,
            api_version,
        )
        .await
    }

    pub(crate) async fn post<T>(
        &self,
        endpoint: &str,
        authorized: bool,
        json_body: Option<&T>,
        form_body: Option<&Vec<(String, String)>>,
        base_url_type: BaseUrlType,
        api_version: APIVersion,
    ) -> Result<Response, RateLimitExceeded>
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
            base_url_type,
            api_version,
        )
        .await
    }

    pub(crate) async fn put<T>(
        &self,
        endpoint: &str,
        authorized: bool,
        json_body: Option<&T>,
        form_body: Option<&Vec<(String, String)>>,
        base_url_type: BaseUrlType,
        api_version: APIVersion,
    ) -> Result<Response, RateLimitExceeded>
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
            base_url_type,
            api_version,
        )
        .await
    }

    pub(crate) async fn delete<T>(
        &self,
        endpoint: &str,
        authorized: bool,
        params: Option<&Vec<(String, String)>>,
        json_body: Option<&T>,
        base_url_type: BaseUrlType,
        api_version: APIVersion,
    ) -> Result<Response, RateLimitExceeded>
    where
        T: Serialize + ?Sized,
    {
        self.request::<T>(
            Method::DELETE,
            endpoint,
            authorized,
            params,
            json_body,
            None,
            base_url_type,
            api_version,
        )
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
        base_url_type: BaseUrlType,
        api_version: APIVersion,
    ) -> Result<Response, RateLimitExceeded>
    where
        T: Serialize + ?Sized,
    {
        let rate_limit_check_result: Option<RateLimitExceeded> =
            self.rate_limiter.check_rate_limit(endpoint).await;
        if rate_limit_check_result.is_some() {
            return Err(rate_limit_check_result.unwrap());
        }
        let url: String = create_url(base_url_type, api_version, endpoint);

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
        Ok(request.send().await.unwrap())
    }

    pub(crate) async fn verify_authorization(&mut self) -> bool {
        let verify_response: Result<SuccessResponse<ProfileResponse>, ErrorResponse> =
            self.get_profile().await.unwrap();
        verify_response.map_or_else(
            |_| {
                info!("Upstox saved access token invalid");
                false
            },
            |_| {
                info!("Using valid access token from file");
                true
            },
        )
    }

    async fn schedule_refresh_instruments(
        scheduler: &JobScheduler,
        shared_api_client: &Arc<Mutex<ApiClient>>,
    ) {
        let shared_api_client_clone: Arc<Mutex<ApiClient>> = Arc::clone(&shared_api_client);
        let job: Job = Job::new_async_tz(
            "0 30 06 * * *",
            FixedOffset::east_opt(19800).unwrap(),
            move |_, _| {
                let api_client: Arc<Mutex<ApiClient>> = Arc::clone(&shared_api_client_clone);
                Box::pin(async move {
                    let mut client: MutexGuard<ApiClient> = api_client.lock().await;
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
        shared_api_client: &Arc<Mutex<ApiClient>>,
        login_config: LoginConfig,
    ) {
        let shared_api_client_clone: Arc<Mutex<ApiClient>> = Arc::clone(&shared_api_client);
        let job: Job = Job::new_async_tz(
            "0 30 03 * * *",
            FixedOffset::east_opt(19800).unwrap(),
            move |_, _| {
                let api_client: Arc<Mutex<ApiClient>> = Arc::clone(&shared_api_client_clone);
                let login_config: LoginConfig = login_config.clone();
                Box::pin(async move {
                    let mut client: MutexGuard<ApiClient> = api_client.lock().await;
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

pub struct WSConnectConfig {
    pub connect_portfolio_stream: bool,
    pub connect_market_data_stream_v3: bool,
    pub portfolio_stream_update_types: Option<HashSet<PortfolioUpdateType>>,
    pub portfolio_feed_callback: Option<Box<dyn FnMut(PortfolioFeedResponse) + Send + Sync>>,
    pub market_data_feed_v3_callback:
        Option<Box<dyn FnMut(MarketDataFeedV3Response) + Send + Sync>>,
}
