use {
    crate::{
        constants::{BASE_URL, UPSTOX_ACCESS_TOKEN_FILENAME},
        models::{
            error_response::ErrorResponse, success_response::SuccessResponse,
            user::profile_response::ProfileResponse,
        },
        utils::{read_value_from_file, write_value_to_file},
    },
    chrono::FixedOffset,
    reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response},
    serde::Serialize,
    std::sync::Arc,
    tokio::sync::{Mutex, MutexGuard},
    tokio_cron_scheduler::{Job, JobScheduler},
};

#[derive(Debug)]
pub struct ApiClient {
    pub client: ReqwestClient,
    pub api_key: String,
    token: Option<String>,
}

impl ApiClient {
    pub async fn new(
        api_key: &str,
        login_config: LoginConfig,
    ) -> Result<Arc<Mutex<ApiClient>>, String> {
        // TODO get instruments
        // TODO integrate websockets and its options
        // TODO test auto login task
        let mut api_client: ApiClient = ApiClient {
            client: ReqwestClient::new(),
            api_key: api_key.to_string(),
            token: None,
        };

        api_client.login(&login_config).await?;

        let shared_api_client: Arc<Mutex<ApiClient>> = Arc::new(Mutex::new(api_client));
        if !login_config.authorize {
            return Ok(shared_api_client);
        }

        if let Some(automate_login_config) = login_config.automate_login_config {
            if automate_login_config.schedule_login {
                Self::schedule_auto_login(&shared_api_client, login_config).await;
            }
        }

        Ok(shared_api_client)
    }

    async fn login(&mut self, login_config: &LoginConfig) -> Result<(), String> {
        if let Ok(access_token) = read_value_from_file(UPSTOX_ACCESS_TOKEN_FILENAME) {
            self.token = Some(access_token);
            if self.verify_authorization().await {
                return Ok(());
            }
        };

        if login_config.automate_login_config.is_none() {
            return Err("Must provide automate_login_config for authorization.".to_string());
        }

        let automate_login_config: &AutomateLoginConfig =
            login_config.automate_login_config.as_ref().unwrap();

        let auth_code: String = self.get_authorization_code(automate_login_config).await?;

        match self.get_token(auth_code.to_string()).await {
            Ok(token_response) => {
                self.token = Some(token_response.access_token);
                write_value_to_file(UPSTOX_ACCESS_TOKEN_FILENAME, &self.token.as_ref().unwrap())
                    .unwrap();
                Ok(())
            }
            Err(error_response) => Err(error_response.errors[0].message.clone()),
        }
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
        let url: String = format!("{}{}", BASE_URL, endpoint);

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

    async fn verify_authorization(&mut self) -> bool {
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

    async fn schedule_auto_login(
        shared_api_client: &Arc<Mutex<ApiClient>>,
        login_config: LoginConfig,
    ) {
        let scheduler: JobScheduler = JobScheduler::new().await.unwrap();
        scheduler.shutdown_on_ctrl_c();

        let shared_api_client_clone: Arc<Mutex<ApiClient>> = Arc::clone(&shared_api_client);
        let job: Job = Job::new_async_tz(
            "0 30 3 * * *",
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
        scheduler.start().await.unwrap();
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
