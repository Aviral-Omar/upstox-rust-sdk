use {crate::models::market_quote::OHLC, serde::Deserialize};

#[derive(Deserialize, Debug)]
pub struct OHLCQuoteResponse {
    pub ohlc: OHLC,
    pub last_price: f64,
    pub instrument_token: String,
}
