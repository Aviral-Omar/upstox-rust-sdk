use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LTPQuotesV3Response {
    pub last_price: f64,
    pub instrument_token: String,
    pub ltq: u64,
    pub volume: u32,
    pub cp: f64,
}
