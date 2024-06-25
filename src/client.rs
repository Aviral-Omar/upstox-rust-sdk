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
}

impl ApiClient {
    pub async fn new(base_url: &str, api_key: &str) -> ApiClient {
        // TODO make auth features optional
        // TODO get instruments
        // TODO integrate websockets and its options
        let mut api_client: ApiClient = ApiClient {
            base_url: base_url.to_string(),
            client: ReqwestClient::new(),
            api_key: api_key.to_string(),
            token: Arc::new(RwLock::new(String::new())),
        };

        if let Ok(access_token) = read_value_from_file(UPSTOX_ACCESS_TOKEN_FILENAME) {
            {
                let mut w: RwLockWriteGuard<String> = api_client.token.write().await;
                *w = access_token.trim().to_string();
            }
            let verify_response: Response =
                Self::get(&api_client, USER_GET_PROFILE_ENDPOINT, true, None).await;
            if verify_response.status().as_u16() == 200 {
                println!("Using valid access token from file");
                return api_client;
            } else {
                println!("Access Token invalid");
            }
        };

        let mut auth_code: String = match read_value_from_file(UPSTOX_AUTH_CODE_FILENAME) {
            Ok(code) => code,
            Err(_) => Self::initiate_auth_flow(&mut api_client).await,
        };

        let mut get_token_status: bool = Self::populate_token(&mut api_client, &auth_code).await;
        if !get_token_status {
            auth_code = Self::initiate_auth_flow(&mut api_client).await;
            get_token_status = Self::populate_token(&mut api_client, &auth_code).await;
            if !get_token_status {
                panic!("Unable to authorize");
            }
        }

        api_client
    }

    async fn initiate_auth_flow(&mut self) -> String {
        let code: String = self.get_authorization_code().await;
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
        auth: bool,
        params: Option<&Vec<(String, String)>>,
    ) -> Response {
        self.request::<()>(Method::GET, endpoint, auth, params, None, None)
            .await
    }

    pub async fn post<T>(
        &self,
        endpoint: &str,
        auth: bool,
        json_body: Option<&T>,
        form_body: Option<&Vec<(String, String)>>,
    ) -> Response
    where
        T: Serialize + ?Sized,
    {
        self.request(Method::POST, endpoint, auth, None, json_body, form_body)
            .await
    }

    pub async fn put<T>(
        &self,
        endpoint: &str,
        auth: bool,
        json_body: Option<&T>,
        form_body: Option<&Vec<(String, String)>>,
    ) -> Response
    where
        T: Serialize + ?Sized,
    {
        self.request(Method::PUT, endpoint, auth, None, json_body, form_body)
            .await
    }

    pub async fn delete(
        &self,
        endpoint: &str,
        auth: bool,
        params: Option<&Vec<(String, String)>>,
    ) -> Response {
        self.request::<()>(Method::DELETE, endpoint, auth, params, None, None)
            .await
    }

    async fn request<T>(
        &self,
        method: Method,
        endpoint: &str,
        auth: bool,
        params: Option<&Vec<(String, String)>>,
        json_body: Option<&T>,
        form_body: Option<&Vec<(String, String)>>,
    ) -> Response
    where
        T: Serialize + ?Sized,
    {
        let url: String = format!("{}{}", self.base_url, endpoint);

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

        if auth {
            let token: RwLockReadGuard<String> = self.token.read().await;
            request = request.bearer_auth(&*token);
        }
        request = request.header("Accept", "application/json");
        request.send().await.unwrap()
    }
}
