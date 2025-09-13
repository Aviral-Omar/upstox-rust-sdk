use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionGreeksResponse {
    pub last_price: f64,
    pub instrument_token: String,
    pub ltq: u64,
    pub volume: u64,
    pub cp: f64,
    pub iv: f64,
    pub vega: f64,
    pub gamma: f64,
    pub theta: f64,
    pub delta: f64,
    pub oi: f64,
}
