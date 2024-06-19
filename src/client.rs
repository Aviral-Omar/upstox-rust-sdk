use {
    reqwest::{Client, Method, RequestBuilder, Response, Url},
    serde::Serialize,
    std::sync::Arc,
    tokio::sync::{RwLock, RwLockReadGuard},
};

pub struct ApiClient {
    base_url: String,
    client: Client,
    token: Arc<RwLock<String>>,
}

impl ApiClient {
    pub fn new(base_url: &str, token: String) -> ApiClient {
        ApiClient {
            base_url: base_url.to_string(),
            client: Client::new(),
            token: Arc::new(RwLock::new(token)),
        }
    }

    pub async fn get(
        &self,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Response, reqwest::Error> {
        self.request::<()>(Method::GET, endpoint, params, None)
            .await
    }

    pub async fn post<T>(
        &self,
        endpoint: &str,
        body: Option<&T>,
    ) -> Result<Response, reqwest::Error>
    where
        T: Serialize + ?Sized,
    {
        self.request(Method::POST, endpoint, None, body).await
    }

    pub async fn put<T>(&self, endpoint: &str, body: Option<&T>) -> Result<Response, reqwest::Error>
    where
        T: Serialize + ?Sized,
    {
        self.request(Method::PUT, endpoint, None, body).await
    }

    pub async fn delete(
        &self,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
    ) -> Result<Response, reqwest::Error> {
        self.request::<()>(Method::DELETE, endpoint, params, None)
            .await
    }

    async fn request<T>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<&[(&str, &str)]>,
        body: Option<&T>,
    ) -> Result<Response, reqwest::Error>
    where
        T: Serialize + ?Sized,
    {
        let token: RwLockReadGuard<String> = self.token.read().await;
        let url: String = format!("{}{}", self.base_url, endpoint);
        let full_url: Url = Url::parse_with_params(&url, params.unwrap_or(&[])).unwrap();
        print!("{}", full_url);

        let mut request: RequestBuilder = match method {
            Method::GET => self.client.get(full_url),
            Method::POST => self.client.post(full_url),
            Method::PUT => self.client.put(full_url),
            Method::DELETE => self.client.delete(full_url),
            _ => unreachable!(),
        };
        if let Some(req_body) = body {
            request = request.json(req_body);
        }

        request = request.bearer_auth(&*token);
        request.send().await
    }
}
