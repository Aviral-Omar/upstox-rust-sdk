use {
    crate::{
        constants::{
            UPSTOX_ACCESS_TOKEN_FILENAME, UPSTOX_AUTH_CODE_FILENAME, USER_GET_PROFILE_ENDPOINT,
        },
        utils::{read_value_from_file, write_value_to_file},
    },
    reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response},
    serde::Serialize,
    std::sync::Arc,
    tokio::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

#[derive(Debug)]
pub struct ApiClient {
    pub base_url: String,
    pub client: ReqwestClient,
    pub api_key: String,
    pub token: Arc<RwLock<String>>,
    pub authorized: bool,
}

impl ApiClient {
    pub async fn new(base_url: &str, api_key: &str, login_config: LoginConfig) -> ApiClient {
        // TODO get instruments
        // TODO integrate websockets and its options
        // TODO auto login task
        let mut api_client: ApiClient = ApiClient {
            base_url: base_url.to_string(),
            client: ReqwestClient::new(),
            api_key: api_key.to_string(),
            token: Arc::new(RwLock::new(String::new())),
            authorized: false,
        };

        if !login_config.authorize {
            return api_client;
        }

        if let Ok(access_token) = read_value_from_file(UPSTOX_ACCESS_TOKEN_FILENAME) {
            {
                let mut w: RwLockWriteGuard<String> = api_client.token.write().await;
                *w = access_token;
            }

            api_client.authorized = true;
            let verify_response: Response =
                Self::get(&api_client, USER_GET_PROFILE_ENDPOINT, true, None).await;
            if verify_response.status().as_u16() == 200 {
                println!("Using valid access token from file");
                return api_client;
            } else {
                api_client.authorized = false;
                println!("Access Token invalid");
            }
        };

        if login_config.automate_login_config.is_none() {
            panic!("Must provide automate_login_config for authorization.")
        }

        let automate_login_config: AutomateLoginConfig =
            login_config.automate_login_config.unwrap();

        let mut auth_code: String = match read_value_from_file(UPSTOX_AUTH_CODE_FILENAME) {
            Ok(code) => code,
            Err(_) => Self::initiate_auth_flow(&mut api_client, &automate_login_config).await,
        };

        let mut get_token_status: bool = Self::populate_token(&mut api_client, &auth_code).await;
        if !get_token_status {
            auth_code = Self::initiate_auth_flow(&mut api_client, &automate_login_config).await;
            get_token_status = Self::populate_token(&mut api_client, &auth_code).await;
            if !get_token_status {
                panic!("Unable to authorize");
            }
        }
        api_client.authorized = true;

        api_client
    }

    async fn initiate_auth_flow(&mut self, automate_login_config: &AutomateLoginConfig) -> String {
        let code: String = self.get_authorization_code(automate_login_config).await;
        write_value_to_file(UPSTOX_AUTH_CODE_FILENAME, &code).unwrap();
        code
    }

    async fn populate_token(&mut self, auth_code: &str) -> bool {
        match self.get_token(auth_code.to_string()).await {
            Ok(token_response) => {
                let mut w: RwLockWriteGuard<String> = self.token.write().await;
                *w = token_response.access_token;
                write_value_to_file(UPSTOX_ACCESS_TOKEN_FILENAME, &w).unwrap();
                true
            }
            Err(error_response) => {
                if error_response.errors[0].error_code == "UDAPI100057" {
                    println!("{}", error_response.errors[0].message);
                    false
                } else {
                    panic!("{:?}", error_response.errors[0].message);
                }
            }
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
        let url: String = format!("{}{}", self.base_url, endpoint);
        if authorized && !self.authorized {
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
            _ => unreachable!(),
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
            let token: RwLockReadGuard<String> = self.token.read().await;
            request = request.bearer_auth(&*token);
        }
        request = request.header("Accept", "application/json");
        request.send().await.unwrap()
    }
}

pub struct LoginConfig {
    pub authorize: bool,
    pub automate_login_config: Option<AutomateLoginConfig>,
}

pub struct AutomateLoginConfig {
    pub automate_login: bool,
    pub automate_fetching_otp: bool,
    pub mail_provider: Option<MailProvider>,
}

#[derive(Clone)]
pub enum MailProvider {
    Google,
}
