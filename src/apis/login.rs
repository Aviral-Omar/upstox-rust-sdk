use {
    crate::{
        client::ApiClient,
        constants::{
            BASE_URL, EMAIL_ID_ENV, GOOGLE_AUTHORIZATION_CODE_ENV, GOOGLE_CLIENT_ID_ENV,
            GOOGLE_CLIENT_SECRET_ENV, GOOGLE_IMAP_URL, GOOGLE_OAUTH2_ACCESS_TOKEN_URL,
            GOOGLE_OAUTH2_AUTH_URL, GOOGLE_REFRESH_TOKEN_FILENAME, LOGIN_AUTHORIZE_ENDPOINT,
            LOGIN_PIN_ENV, MOBILE_NUMBER_ENV, REDIRECT_PORT_ENV,
        },
        models::login::{
            dialog_request::{DialogRequest, ResponseType},
            google_oauth2_request::{
                self, AccessType, GoogleOAuth2AuthRequest, GoogleOAuth2CodeTokenRequest,
                GoogleOAuth2RefreshTokenRequest, GrantType, Prompt,
            },
            google_oauth2_response::{GoogleOAuth2TokenErrorResponse, GoogleOAuth2TokenResponse},
        },
        utils::{read_value_from_file, write_value_to_file, ToQueryParams},
    },
    async_imap::{
        self,
        types::{Fetch, Mailbox},
        Authenticator, Client as ImapClient, Session,
    },
    async_native_tls::{TlsConnector, TlsStream},
    chrono::{DateTime, Utc},
    fantoccini::{elements::Element, Client, Locator},
    futures::{Future, TryStreamExt},
    mailparse::{parse_mail, ParsedMail},
    regex::Regex,
    reqwest::Url,
    scraper::{ElementRef, Html, Selector},
    std::{self, borrow::Cow, env, fs::remove_file, net::SocketAddr, pin::Pin, sync::Arc},
    tokio::{
        self,
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
        sync::Mutex,
        time::{sleep, Duration},
    },
    url_open::UrlOpen,
    urlencoding::decode,
};

#[derive(Debug)]
struct OAuth2 {
    user: String,
    access_token: String,
}

impl Authenticator for &OAuth2 {
    type Response = String;
    fn process(&mut self, _: &[u8]) -> Self::Response {
        format!(
            "user={}\x01auth=Bearer {}\x01\x01",
            self.user, self.access_token
        )
    }
}

pub async fn get_authorization_code(
    api_client: &ApiClient,
    fantoccini_client: Arc<Mutex<Option<Client>>>,
) -> String {
    let redirect_port: String = env::var(REDIRECT_PORT_ENV).unwrap();
    let login_pin: String = env::var(LOGIN_PIN_ENV).unwrap();

    let dialog_request_params: DialogRequest = DialogRequest {
        client_id: api_client.api_key.clone(),
        redirect_uri: format!("{}{}", "http://127.0.0.1:", &redirect_port),
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
        client.goto(full_url.as_str()).await.unwrap();
    }

    sleep(Duration::from_millis(200)).await;
    let otp_sent_timestamp: i64 = Utc::now().timestamp();
    send_otp(fantoccini_client.clone()).await;
    // sleep(Duration::from_millis(5000)).await;

    let otp: String = get_otp(fantoccini_client.clone(), otp_sent_timestamp).await;
    {
        let mut client: tokio::sync::MutexGuard<Option<Client>> = fantoccini_client.lock().await;
        let client: &mut Client = client.as_mut().expect("Client is already closed");
        let otp_field: Element = client
            .wait()
            .every(Duration::from_millis(100))
            .for_element(Locator::Id("otpNum"))
            .await
            .unwrap();
        otp_field.send_keys(&otp).await.unwrap();

        let continue_button: Element = client.find(Locator::Id("continueBtn")).await.unwrap();
        continue_button.click().await.unwrap();

        let pin_field: Element = client
            .wait()
            .every(Duration::from_millis(100))
            .for_element(Locator::Id("pinCode"))
            .await
            .unwrap();
        pin_field.send_keys(&login_pin).await.unwrap();

        let pin_continue_button: Element =
            client.find(Locator::Id("pinContinueBtn")).await.unwrap();

        let code_future = await_and_extract_code();
        pin_continue_button.click().await.unwrap();

        code_future.await
    }
}

async fn send_otp(fantoccini_client: Arc<Mutex<Option<Client>>>) {
    let mobile_number: String = env::var(MOBILE_NUMBER_ENV).unwrap();
    {
        let client: tokio::sync::MutexGuard<Option<Client>> = fantoccini_client.lock().await;
        let client: &Client = client.as_ref().expect("Client is already closed");
        let mobile_number_field: Element = client.find(Locator::Id("mobileNum")).await.unwrap();
        mobile_number_field.send_keys(&mobile_number).await.unwrap();

        let get_otp_button: Element = client.find(Locator::Id("getOtp")).await.unwrap();
        get_otp_button.click().await.unwrap();
    }
}

async fn get_otp(fantoccini_client: Arc<Mutex<Option<Client>>>, otp_sent_time: i64) -> String {
    let email: String = env::var(EMAIL_ID_ENV).unwrap();
    let access_token: String = get_google_access_token(fantoccini_client.clone()).await;

    let oauth2: OAuth2 = OAuth2 {
        user: email,
        access_token,
    };
    let domain: &str = GOOGLE_IMAP_URL;
    let tcp_stream: TcpStream = TcpStream::connect((domain, 993)).await.unwrap();
    let tls_connector: TlsConnector = TlsConnector::new();
    let tls_stream: TlsStream<TcpStream> = tls_connector.connect(domain, tcp_stream).await.unwrap();

    let mut client: ImapClient<TlsStream<TcpStream>> = ImapClient::new(tls_stream);
    let _greeting = client
        .read_response()
        .await
        .expect("Unexpected end of stream, expected greeting");

    let mut imap_session: Session<TlsStream<TcpStream>> =
        client.authenticate("XOAUTH2", &oauth2).await.unwrap();

    println!("OTP Sent: {}", otp_sent_time);
    loop {
        let inbox: Mailbox = imap_session.select("INBOX").await.unwrap();
        let msg_count: u32 = inbox.exists;

        for seq_no in ((msg_count - 2)..=msg_count).rev() {
            let msg_headers: Option<Fetch> = get_message_data(
                &mut imap_session,
                seq_no.to_string(),
                "BODY[HEADER.FIELDS (SUBJECT FROM DATE)]",
            )
            .await;
            let msg_headers: Fetch = match msg_headers {
                Some(msg_headers) => msg_headers,
                None => break,
            };
            let headers: &str = std::str::from_utf8(msg_headers.header().unwrap()).unwrap();

            let msg_timestamp: i64 =
                DateTime::parse_from_rfc2822(get_header(headers, "Date").unwrap().as_str())
                    .unwrap()
                    .timestamp();
            println!("Message Time: {}", msg_timestamp);
            if msg_timestamp < otp_sent_time {
                break;
            }

            let from_match: bool = check_header(
                headers,
                "From",
                "Upstox Alert <donotreply@transactions.upstox.com>",
            );
            let subject_match: bool = check_header(headers, "Subject", "OTP to login");

            if from_match && subject_match {
                let msg_text: Fetch =
                    get_message_data(&mut imap_session, seq_no.to_string(), "BODY[TEXT]")
                        .await
                        .unwrap();
                imap_session.logout().await.unwrap();

                let raw_text: &[u8] = msg_text.text().unwrap();
                let parsed_mail: ParsedMail = parse_mail(raw_text).unwrap();
                let html_content: String = parsed_mail.get_body().unwrap();

                let document: Html = Html::parse_document(&html_content);
                let span_selector: Selector = Selector::parse("span").unwrap();

                let re: Regex = Regex::new(r"[0-9]{6}").unwrap();
                let otp_element: ElementRef = document
                    .select(&span_selector)
                    .into_iter()
                    .find(|element| match element.text().next() {
                        Some(val) => re.find(val).is_some(),
                        None => false,
                    })
                    .unwrap();

                let otp: &str = otp_element.text().next().unwrap();

                return otp.to_string();
            }
        }
        sleep(Duration::from_millis(1000)).await;
    }
}

async fn get_message_data(
    imap_session: &mut Session<TlsStream<TcpStream>>,
    seq_set: String,
    query: &str,
) -> Option<Fetch> {
    let msgs_stream = imap_session.fetch(seq_set, query).await.unwrap();
    let msgs: Vec<Fetch> = msgs_stream.try_collect().await.unwrap();
    msgs.into_iter().next()
}

fn get_header(headers: &str, field: &str) -> Option<String> {
    for line in headers.lines() {
        if line.to_lowercase().starts_with(&field.to_lowercase()) {
            return Some(line[field.len() + 1..].trim().to_string());
        }
    }
    None
}

fn check_header(headers: &str, field: &str, expected_value: &str) -> bool {
    match get_header(headers, field) {
        Some(value) => value == expected_value,
        None => false,
    }
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
        match read_value_from_file(GOOGLE_REFRESH_TOKEN_FILENAME) {
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

                let redirect_port: String = env::var(REDIRECT_PORT_ENV).unwrap();

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
                    let _ = write_value_to_file(
                        GOOGLE_REFRESH_TOKEN_FILENAME,
                        response_data.refresh_token.unwrap().as_str(),
                    );
                }
                return response_data.access_token;
            }
            400 => {
                let error_data: GoogleOAuth2TokenErrorResponse =
                    res.json::<GoogleOAuth2TokenErrorResponse>().await.unwrap();
                if refresh_token_found {
                    let _ = remove_file(GOOGLE_REFRESH_TOKEN_FILENAME);
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
    let redirect_port: String = env::var(REDIRECT_PORT_ENV).unwrap();

    let google_oauth2_auth_request: GoogleOAuth2AuthRequest = GoogleOAuth2AuthRequest {
        client_id,
        redirect_uri: format!("{}{}", "http://127.0.0.1:", &redirect_port),
        response_type: google_oauth2_request::ResponseType::Code,
        scope: "https://mail.google.com/".to_string(),
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

    await_and_extract_code().await
}

async fn await_and_extract_code() -> String {
    let redirect_port: String = env::var(REDIRECT_PORT_ENV).unwrap();

    let addr: SocketAddr =
        SocketAddr::from(([127, 0, 0, 1], str::parse::<u16>(&redirect_port).unwrap()));
    let listener: TcpListener = TcpListener::bind(addr).await.unwrap();
    let (mut socket, _) = listener.accept().await.unwrap();
    let mut buffer: [u8; 1024] = [0; 1024];
    socket.read(&mut buffer).await.unwrap();
    let request: Cow<str> = String::from_utf8_lossy(&buffer[..]);

    let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<!DOCTYPE html><html><body>You can now close this tab!</body></html>";
    socket.write_all(response.as_bytes()).await.unwrap();
    socket.flush().await.unwrap();
    socket.shutdown().await.unwrap();
    parse_code(request.to_string()).unwrap()
}

fn parse_code(request: String) -> Option<String> {
    if let Some(start_index) = request.find("code=") {
        let start_index: usize = start_index + 5;
        if let Some(end_index) = request[start_index..].find(|c| c == '&' || c == ' ') {
            let end_index: usize = start_index + end_index;
            let code: &str = &request[start_index..end_index];
            return decode(code).ok().map(|decoded| decoded.into_owned());
        }
    }

    None
}
