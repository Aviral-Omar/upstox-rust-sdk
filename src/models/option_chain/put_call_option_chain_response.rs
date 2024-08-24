use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionMarketData {
    pub ltp: f64,
    pub close_price: f64,
    pub volume: u64,
    pub oi: f64,
    pub bid_price: f64,
    pub bid_qty: u64,
    pub ask_price: f64,
    pub ask_qty: u64,
    pub prev_oi: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionGreeks {
    pub vega: f64,
    pub theta: f64,
    pub gamma: f64,
    pub delta: f64,
    pub iv: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionData {
    pub instrument_key: String,
    pub market_data: OptionMarketData,
    pub option_greeks: OptionGreeks,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionChainResponse {
    pub expiry: String,
    pub pcr: Option<f64>,
    pub strike_price: f64,
    pub underlying_key: String,
    pub underlying_spot_price: f64,
    pub call_options: OptionData,
    pub put_options: OptionData,
}
