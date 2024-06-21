use {
    crate::{
        client::ApiClient,
        constants::{
            BASE_URL, GOOGLE_AUTHORIZATION_CODE_ENV, GOOGLE_CLIENT_ID_ENV,
            GOOGLE_CLIENT_SECRET_ENV, GOOGLE_OAUTH2_ACCESS_TOKEN_URL, GOOGLE_OAUTH2_AUTH_URL,
            GOOGLE_REDIRECT_PORT_ENV, LOGIN_AUTHORIZE_ENDPOINT, LOGIN_PIN_ENV, MOBILE_NUMBER_ENV,
            UPSTOX_REDIRECT_URI_ENV,
        },
        models::login::{
            dialog_request::{DialogRequest, ResponseType},
            google_oauth2_request::{
                self, AccessType, GoogleOAuth2AuthRequest, GoogleOAuth2CodeTokenRequest,
                GoogleOAuth2RefreshTokenRequest, GrantType, Prompt,
            },
            google_oauth2_response::{GoogleOAuth2TokenErrorResponse, GoogleOAuth2TokenResponse},
        },
        utils::ToQueryParams,
    },
    fantoccini::{elements::Element, Client, Locator},
    futures::Future,
    reqwest::Url,
    std::{
        borrow::Cow,
        env,
        fs::{remove_file, File},
        io::{Read, Write},
        net::SocketAddr,
        pin::Pin,
        sync::Arc,
    },
    tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
        sync::Mutex,
        time::{sleep, Duration},
    },
    url_open::UrlOpen,
    urlencoding::decode,
};

pub async fn get_authorization_code(
    client: &ApiClient,
    fantoccini_client: Arc<Mutex<Option<Client>>>,
) {
    let redirect_uri: String = env::var(UPSTOX_REDIRECT_URI_ENV).unwrap();

    let dialog_request_params: DialogRequest = DialogRequest {
        client_id: client.api_key.clone(),
        redirect_uri,
        state: None,
        response_type: ResponseType::Code,
    };
    let full_url: Url = Url::parse_with_params(
        format!("{}{}", BASE_URL, LOGIN_AUTHORIZE_ENDPOINT).as_str(),
        dialog_request_params.to_query_params(),
    )
    .unwrap();

    {
        let mut client: tokio::sync::MutexGuard<Option<Client>> = fantoccini_client.lock().await;
        let client: &mut Client = client.as_mut().expect("Client is already closed");
        client.minimize_window().await.unwrap();
        client.goto(full_url.as_str()).await.unwrap();
    }

    sleep(Duration::from_millis(1000)).await;
    send_otp(fantoccini_client.clone()).await;

    sleep(Duration::from_millis(5000)).await;
}

async fn send_otp(fantoccini_client: Arc<Mutex<Option<Client>>>) {
    let mobile_number: String = env::var(MOBILE_NUMBER_ENV).unwrap();
    let login_pin: String = env::var(LOGIN_PIN_ENV).unwrap();
    {
        let client: tokio::sync::MutexGuard<Option<Client>> = fantoccini_client.lock().await;
        let client: &Client = client.as_ref().expect("Client is already closed");
        let mobile_number_field: Element = client.find(Locator::Id("mobileNum")).await.unwrap();
        mobile_number_field.send_keys(&mobile_number).await.unwrap();

        let get_otp_button: Element = client.find(Locator::Id("getOtp")).await.unwrap();
        // TODO get_otp_button.click().await.unwrap();
    }
    let otp: String = get_otp(fantoccini_client.clone()).await;

    sleep(Duration::from_millis(2000)).await;
}

async fn get_otp(fantoccini_client: Arc<Mutex<Option<Client>>>) -> String {
    let access_token: String = get_google_access_token(fantoccini_client.clone()).await;
    access_token
    // TODO
}

fn get_google_access_token(
    fantoccini_client: Arc<Mutex<Option<Client>>>,
) -> Pin<Box<dyn Future<Output = String>>> {
    Box::pin(async move {
        let client: reqwest::Client = reqwest::Client::new();

        let client_id: String = env::var(GOOGLE_CLIENT_ID_ENV).unwrap();
        let client_secret: String = env::var(GOOGLE_CLIENT_SECRET_ENV).unwrap();

        let google_oauth2_token_request_body: Box<dyn ToQueryParams>;
        let refresh_token_found: bool;
        match read_value_from_file() {
            // Refresh Token already present, so use it to get new access token
            Ok(refresh_token) => {
                refresh_token_found = true;
                google_oauth2_token_request_body = Box::new(GoogleOAuth2RefreshTokenRequest {
                    client_id,
                    client_secret,
                    grant_type: GrantType::RefreshToken,
                    refresh_token,
                });
            }
            // No refresh token found, so use freshly generated authorization code from environment to generate access_token and refresh_token
            Err(_) => {
                refresh_token_found = false;
                let code: String = match env::var(GOOGLE_AUTHORIZATION_CODE_ENV) {
                    Ok(code) => code,
                    Err(_) => get_google_auth_code().await,
                };

                let redirect_port: String = env::var(GOOGLE_REDIRECT_PORT_ENV).unwrap();

                google_oauth2_token_request_body = Box::new(GoogleOAuth2CodeTokenRequest {
                    client_id,
                    client_secret,
                    code,
                    code_verifier: None,
                    grant_type: GrantType::AuthorizationCode,
                    redirect_uri: format!("{}{}", "http://127.0.0.1:", &redirect_port),
                });
            }
        }

        let res: reqwest::Response = client
            .post(GOOGLE_OAUTH2_ACCESS_TOKEN_URL)
            .form(&google_oauth2_token_request_body.to_query_params())
            .send()
            .await
            .unwrap();

        match res.status().as_u16() {
            200 => {
                let response_data = res.json::<GoogleOAuth2TokenResponse>().await.unwrap();
                if !refresh_token_found {
                    let _ = write_value_to_file(response_data.refresh_token.unwrap().as_str());
                }
                return response_data.access_token;
            }
            400 => {
                let error_data: GoogleOAuth2TokenErrorResponse =
                    res.json::<GoogleOAuth2TokenErrorResponse>().await.unwrap();
                if refresh_token_found {
                    let _ = remove_file("refresh_token.txt");
                }
                print!("{}", error_data.error);
                return get_google_access_token(fantoccini_client).await;
            }
            _ => panic!(),
        };
    })
}

async fn get_google_auth_code() -> String {
    let client_id: String = env::var(GOOGLE_CLIENT_ID_ENV).unwrap();
    let redirect_port: String = env::var(GOOGLE_REDIRECT_PORT_ENV).unwrap();

    let google_oauth2_auth_request: GoogleOAuth2AuthRequest = GoogleOAuth2AuthRequest {
        client_id,
        redirect_uri: format!("{}{}", "http://127.0.0.1:", &redirect_port),
        response_type: google_oauth2_request::ResponseType::Code,
        scope: "https://www.googleapis.com/auth/gmail.readonly".to_string(),
        code_challenge: None,
        code_challenge_method: None,
        state: None,
        login_hint: None,
        access_type: Some(AccessType::Offline),
        prompt: Some(Prompt::SelectAccount),
    };

    let oauth_url: Url = Url::parse_with_params(
        GOOGLE_OAUTH2_AUTH_URL,
        google_oauth2_auth_request.to_query_params(),
    )
    .unwrap();
    oauth_url.open();

    let addr: SocketAddr =
        SocketAddr::from(([127, 0, 0, 1], str::parse::<u16>(&redirect_port).unwrap()));
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();
    let (mut socket, _) = listener.accept().await.unwrap();
    handle_connection(&mut socket).await
}

async fn handle_connection(socket: &mut TcpStream) -> String {
    let mut buffer: [u8; 1024] = [0; 1024];
    socket.read(&mut buffer).await.unwrap();
    let request: Cow<str> = String::from_utf8_lossy(&buffer[..]);
    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><html><body><script>window.close();</script></body></html>";
    socket.write_all(response.as_bytes()).await.unwrap();
    socket.flush().await.unwrap();
    socket.shutdown().await.unwrap();
    parse_code(request.to_string()).unwrap()
}

fn parse_code(request: String) -> Option<String> {
    if let Some(start_index) = request.find("code=") {
        let start_index: usize = start_index + 5;
        if let Some(end_index) = request[start_index..].find("&scope") {
            let end_index: usize = start_index + end_index;
            let code: &str = &request[start_index..end_index];
            return decode(code).ok().map(|decoded| decoded.into_owned());
        }
    }

    None
}

fn write_value_to_file(value: &str) -> std::io::Result<()> {
    let mut file: File = File::create("refresh_token.txt")?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

fn read_value_from_file() -> std::io::Result<String> {
    let mut file: File = File::open("refresh_token.txt")?;
    let mut value: String = String::new();
    file.read_to_string(&mut value)?;
    Ok(value)
}
