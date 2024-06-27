pub const BASE_URL: &str = "https://api-v2.upstox.com";

pub const LOGIN_AUTHORIZE_ENDPOINT: &str = "/login/authorization/dialog";
pub const LOGIN_GET_TOKEN_ENDPOINT: &str = "/login/authorization/token";
pub const LOGOUT_ENDPOINT: &str = "/logout";

pub const INSTRUMENTS_COMPLETE_URL: &str =
    "https://assets.upstox.com/market-quote/instruments/exchange/complete.json.gz";

pub const USER_GET_FUND_AND_MARGIN_ENDPOINT: &str = "/user/get-funds-and-margin";
pub const USER_GET_PROFILE_ENDPOINT: &str = "/user/profile";

pub const CHARGES_BROKERAGE_DETAILS_ENDPOINT: &str = "/charges/brokerage";

pub const ORDERS_PLACE_ORDER_ENDPOINT: &str = "/order/place";
pub const ORDERS_MODIFY_ORDER_ENDPOINT: &str = "/order/modify";
pub const ORDERS_CANCEL_ORDER_ENDPOINT: &str = "/order/cancel";
pub const ORDERS_ORDER_DETAILS_ENDPOINT: &str = "/order/details";
pub const ORDERS_ORDER_HISTORY_ENDPOINT: &str = "/order/history";
pub const ORDERS_ORDER_BOOK_ENDPOINT: &str = "/order/retrieve-all";
pub const ORDERS_TRADES_ENDPOINT: &str = "/order/get-trades-for-day";
pub const ORDERS_ORDER_TRADES_ENDPOINT: &str = "/order/trades";
pub const ORDERS_TRADE_HISTORY_ENDPOINT: &str = "/charges/historical-trades";

pub const TRADE_PNL_REPORT_METADATA_ENDPOINT: &str = "/trade/profit-loss/metadata";
pub const TRADE_PNL_REPORT_ENDPOINT: &str = "/trade/profit-loss/data";
pub const TRADE_PNL_TRADES_CHARGES_ENDPOINT: &str = "/trade/profit-loss/charges";

pub const HISTORICAL_CANDLE_DATA_ENDPOINT: &str = "/historical-candle";
pub const HISTORICAL_CANDLE_INTRADAY_DATA_ENDPOINT: &str = "/historical-candle/intraday";

pub const PORTFOLIO_POSITIONS_ENDPOINT: &str = "/portfolio/short-term-positions";
pub const PORTFOLIO_CONVERT_POSITIONS_ENDPOINT: &str = "/portfolio/convert-position";
pub const PORTFOLIO_HOLDINGS_ENDPOINT: &str = "/portfolio/long-term-holdings";

pub const MARKET_QUOTE_FULL_ENDPOINT: &str = "/market-quote/quotes";
pub const MARKET_QUOTE_OHLC_ENDPOINT: &str = "/market-quote/ohlc";
pub const MARKET_QUOTE_LTP_ENDPOINT: &str = "/market-quote/ltp";

pub const MARKET_INFO_HOLIDAYS_ENDPOINT: &str = "/market/holidays";
pub const MARKET_INFO_TIMINGS_ENDPOINT: &str = "/market/timings";
pub const MARKET_INFO_EXCHANGE_STATUS_ENDPOINT: &str = "/market/status";

pub const OPTION_CONTRACTS_ENDPOINT: &str = "/option/contract";
pub const OPTION_CHAIN_ENDPOINT: &str = "/option/chain";

pub const GOOGLE_IMAP_URL: &str = "imap.gmail.com";
pub const GOOGLE_OAUTH2_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const GOOGLE_OAUTH2_ACCESS_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

pub const GOOGLE_REFRESH_TOKEN_FILENAME: &str = "refresh_token.txt";
pub const UPSTOX_ACCESS_TOKEN_FILENAME: &str = "access_token.txt";
pub const INSTRUMENTS_ARCHIVE_FILENAME: &str = "complete.json.gz";
pub const INSTRUMENTS_JSON_FILENAME: &str = "complete.json";

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
