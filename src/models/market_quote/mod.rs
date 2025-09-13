pub mod full_market_quotes_request;
pub mod full_market_quotes_response;
pub mod ltp_quotes_request;
pub mod ltp_quotes_response;
pub mod ltp_quotes_v3_response;
pub mod ohlc_quotes_request;
pub mod ohlc_quotes_response;
pub mod ohlc_quotes_v3_response;
pub mod option_greeks_request;
pub mod option_greeks_response;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OHLC {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OHLCV3 {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u32,
    pub ts: u64,
}
