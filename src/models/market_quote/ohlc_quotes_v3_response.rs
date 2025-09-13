use {
    crate::models::market_quote::OHLCV3,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct OHLCQuotesV3Response {
    pub last_price: f64,
    pub instrument_token: String,
    pub prev_ohlc: OHLCV3,
    pub live_ohlc: OHLCV3,
}
