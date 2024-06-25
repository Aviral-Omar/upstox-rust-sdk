use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CandleDataResponse {
    // Order is timestamp, open, high, low, close, volume, open interest
    pub candles: Vec<(String, f64, f64, f64, f64, i32, u32)>,
}
