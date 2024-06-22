use {
    crate::{
        apis::login::get_authorization_code,
        constants::{UPSTOX_AUTH_TOKEN_FILENAME, WEBDRIVER_SOCKET_ENV},
        utils::{read_value_from_file, write_value_to_file},
    },
    fantoccini::{Client as FantocciniClient, ClientBuilder},
    reqwest::{Client as ReqwestClient, Method, RequestBuilder, Response},
    serde::Serialize,
    std::{env, sync::Arc},
    tokio::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard},
};

pub struct ApiClient {
    pub base_url: String,
    pub client: ReqwestClient,
    pub api_key: String,
    pub token: Arc<RwLock<String>>,
}

impl ApiClient {
    pub async fn new(base_url: &str, api_key: &str) -> ApiClient {
        let webdriver_socket: String = env::var(WEBDRIVER_SOCKET_ENV).unwrap();

        let api_client: ApiClient = ApiClient {
            base_url: base_url.to_string(),
            client: ReqwestClient::new(),
            api_key: api_key.to_string(),
            token: Arc::new(RwLock::new(String::new())),
        };
        let fantoccini_client: FantocciniClient = ClientBuilder::native()
            .connect(&webdriver_socket)
            .await
            .expect("Failed to connect to WebDriver");
        let fantoccini_client: Arc<Mutex<Option<FantocciniClient>>> =
            Arc::new(Mutex::new(Some(fantoccini_client)));

        let auth_code: String = match read_value_from_file(UPSTOX_AUTH_TOKEN_FILENAME) {
            Ok(code) => code,
            Err(_) => {
                let code: String =
                    get_authorization_code(&api_client, fantoccini_client.clone()).await;
                write_value_to_file(UPSTOX_AUTH_TOKEN_FILENAME, &code).unwrap();
                code
            }
        };

        close_fantoccini_client(fantoccini_client).await;
        api_client
    }

    pub async fn get(
        &self,
        endpoint: &str,
        auth: bool,
        params: Option<Vec<(String, String)>>,
    ) -> Response {
        self.request::<()>(Method::GET, endpoint, auth, params, None)
            .await
    }

    pub async fn post<T>(&self, endpoint: &str, auth: bool, body: Option<&T>) -> Response
    where
        T: Serialize + ?Sized,
    {
        self.request(Method::POST, endpoint, auth, None, body).await
    }

    pub async fn put<T>(&self, endpoint: &str, auth: bool, body: Option<&T>) -> Response
    where
        T: Serialize + ?Sized,
    {
        self.request(Method::PUT, endpoint, auth, None, body).await
    }

    pub async fn delete(
        &self,
        endpoint: &str,
        auth: bool,
        params: Option<Vec<(String, String)>>,
    ) -> Response {
        self.request::<()>(Method::DELETE, endpoint, auth, params, None)
            .await
    }

    async fn request<T>(
        &self,
        method: Method,
        endpoint: &str,
        auth: bool,
        params: Option<Vec<(String, String)>>,
        body: Option<&T>,
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
            request = request.form(&req_params);
        }

        if let Some(req_body) = body {
            request = request.json(req_body);
        }

        if auth {
            let token: RwLockReadGuard<String> = self.token.read().await;
            request = request.bearer_auth(&*token);
        }
        request.send().await.unwrap()
    }
}

async fn close_fantoccini_client(fantoccini_client: Arc<Mutex<Option<FantocciniClient>>>) {
    let mut client: MutexGuard<Option<FantocciniClient>> = fantoccini_client.lock().await;
    if let Some(client) = client.take() {
        client.close().await.unwrap();
    }
}
