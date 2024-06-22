pub const BASE_URL: &str = "https://api-v2.upstox.com";

pub const LOGIN_AUTHORIZE_ENDPOINT: &str = "/login/authorization/dialog";
pub const LOGIN_GET_TOKEN_ENDPOINT: &str = "/login/authorization/token";

pub const USER_GET_PROFILE_ENDPOINT: &str = "/user/profile";

pub const GOOGLE_IMAP_URL: &str = "imap.gmail.com";
pub const GOOGLE_OAUTH2_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const GOOGLE_OAUTH2_ACCESS_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

pub const GOOGLE_REFRESH_TOKEN_FILENAME: &str = "refresh_token.txt";
pub const UPSTOX_ACCESS_TOKEN_FILENAME: &str = "access_token.txt";
pub const UPSTOX_AUTH_CODE_FILENAME: &str = "auth_code.txt";

pub const EMAIL_ID_ENV: &str = "EMAIL_ID";
pub const GOOGLE_AUTHORIZATION_CODE_ENV: &str = "GOOGLE_AUTHORIZATION_CODE";
pub const GOOGLE_CLIENT_ID_ENV: &str = "GOOGLE_CLIENT_ID";
pub const GOOGLE_CLIENT_SECRET_ENV: &str = "GOOGLE_CLIENT_SECRET";
pub const MOBILE_NUMBER_ENV: &str = "MOBILE_NUMBER";
pub const LOGIN_PIN_ENV: &str = "LOGIN_PIN";
pub const REDIRECT_PORT_ENV: &str = "REDIRECT_PORT";
pub const UPLINK_API_KEY_ENV: &str = "UPLINK_API_KEY";
pub const UPLINK_API_SECRET_ENV: &str = "UPLINK_API_SECRET";
pub const WEBDRIVER_SOCKET_ENV: &str = "WEBDRIVER_SOCKET";
