pub(super) const REST_BASE_URL: &str = "https://api-v2.upstox.com";

pub(super) const LOGIN_AUTHORIZE_ENDPOINT: &str = "/login/authorization/dialog";
pub(super) const LOGIN_GET_TOKEN_ENDPOINT: &str = "/login/authorization/token";
pub(super) const LOGOUT_ENDPOINT: &str = "/logout";

pub(super) const INSTRUMENTS_COMPLETE_URL: &str =
    "https://assets.upstox.com/market-quote/instruments/exchange/complete.json.gz";

pub(super) const USER_GET_FUND_AND_MARGIN_ENDPOINT: &str = "/user/get-funds-and-margin";
pub(super) const USER_GET_PROFILE_ENDPOINT: &str = "/user/profile";

pub(super) const CHARGES_BROKERAGE_DETAILS_ENDPOINT: &str = "/charges/brokerage";

pub(super) const ORDERS_PLACE_ORDER_ENDPOINT: &str = "/order/place";
pub(super) const ORDERS_MODIFY_ORDER_ENDPOINT: &str = "/order/modify";
pub(super) const ORDERS_CANCEL_ORDER_ENDPOINT: &str = "/order/cancel";
pub(super) const ORDERS_ORDER_DETAILS_ENDPOINT: &str = "/order/details";
pub(super) const ORDERS_ORDER_HISTORY_ENDPOINT: &str = "/order/history";
pub(super) const ORDERS_ORDER_BOOK_ENDPOINT: &str = "/order/retrieve-all";
pub(super) const ORDERS_TRADES_ENDPOINT: &str = "/order/get-trades-for-day";
pub(super) const ORDERS_ORDER_TRADES_ENDPOINT: &str = "/order/trades";
pub(super) const ORDERS_TRADE_HISTORY_ENDPOINT: &str = "/charges/historical-trades";

pub(super) const TRADE_PNL_REPORT_METADATA_ENDPOINT: &str = "/trade/profit-loss/metadata";
pub(super) const TRADE_PNL_REPORT_ENDPOINT: &str = "/trade/profit-loss/data";
pub(super) const TRADE_PNL_TRADES_CHARGES_ENDPOINT: &str = "/trade/profit-loss/charges";

pub(super) const HISTORICAL_CANDLE_DATA_ENDPOINT: &str = "/historical-candle";
pub(super) const HISTORICAL_CANDLE_INTRADAY_DATA_ENDPOINT: &str = "/historical-candle/intraday";

pub(super) const PORTFOLIO_POSITIONS_ENDPOINT: &str = "/portfolio/short-term-positions";
pub(super) const PORTFOLIO_CONVERT_POSITIONS_ENDPOINT: &str = "/portfolio/convert-position";
pub(super) const PORTFOLIO_HOLDINGS_ENDPOINT: &str = "/portfolio/long-term-holdings";

pub(super) const MARKET_QUOTE_FULL_ENDPOINT: &str = "/market-quote/quotes";
pub(super) const MARKET_QUOTE_OHLC_ENDPOINT: &str = "/market-quote/ohlc";
pub(super) const MARKET_QUOTE_LTP_ENDPOINT: &str = "/market-quote/ltp";

pub(super) const MARKET_INFO_HOLIDAYS_ENDPOINT: &str = "/market/holidays";
pub(super) const MARKET_INFO_TIMINGS_ENDPOINT: &str = "/market/timings";
pub(super) const MARKET_INFO_EXCHANGE_STATUS_ENDPOINT: &str = "/market/status";

pub(super) const OPTION_CONTRACTS_ENDPOINT: &str = "/option/contract";
pub(super) const OPTION_CHAIN_ENDPOINT: &str = "/option/chain";

pub(super) const WS_PORTFOLIO_FEED_AUTHORIZE_ENDPOINT: &str =
    "/feed/portfolio-stream-feed/authorize";
pub(super) const WS_MARKET_DATA_FEED_AUTHORIZE_ENDPOINT: &str = "/feed/market-data-feed/authorize";

pub(super) const GOOGLE_IMAP_URL: &str = "imap.gmail.com";
pub(super) const GOOGLE_OAUTH2_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
pub(super) const GOOGLE_OAUTH2_ACCESS_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

pub(super) const GOOGLE_REFRESH_TOKEN_FILENAME: &str = "refresh_token.txt";
pub(super) const UPSTOX_ACCESS_TOKEN_FILENAME: &str = "access_token.txt";
pub(super) const INSTRUMENTS_ARCHIVE_FILENAME: &str = "complete.json.gz";
pub(super) const INSTRUMENTS_JSON_FILENAME: &str = "complete.json";

pub(super) const EMAIL_ID_ENV: &str = "EMAIL_ID";
pub(super) const GOOGLE_AUTHORIZATION_CODE_ENV: &str = "GOOGLE_AUTHORIZATION_CODE";
pub(super) const GOOGLE_CLIENT_ID_ENV: &str = "GOOGLE_CLIENT_ID";
pub(super) const GOOGLE_CLIENT_SECRET_ENV: &str = "GOOGLE_CLIENT_SECRET";
pub(super) const MOBILE_NUMBER_ENV: &str = "MOBILE_NUMBER";
pub(super) const LOGIN_PIN_ENV: &str = "LOGIN_PIN";
pub(super) const REDIRECT_PORT_ENV: &str = "REDIRECT_PORT";
pub const UPLINK_API_KEY_ENV: &str = "UPLINK_API_KEY";
pub(super) const UPLINK_API_SECRET_ENV: &str = "UPLINK_API_SECRET";
pub(super) const WEBDRIVER_SOCKET_ENV: &str = "WEBDRIVER_SOCKET";
