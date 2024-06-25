use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LTPQuoteResponse {
    pub last_price: f64,
    pub instrument_token: String,
}
