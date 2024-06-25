use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LTPQuoteResponse {
    pub last_price: f64,
    pub instrument_token: String,
}
