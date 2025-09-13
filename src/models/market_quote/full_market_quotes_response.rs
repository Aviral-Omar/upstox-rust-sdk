use {
    crate::models::market_quote::OHLC,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct DepthLevel {
    pub quantity: u32,
    pub price: f64,
    pub orders: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Depth {
    pub buy: Vec<DepthLevel>,
    pub sell: Vec<DepthLevel>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FullMarketQuotesResponse {
    pub ohlc: OHLC,
    pub depth: Depth,
    pub timestamp: String,
    pub instrument_token: String,
    pub symbol: String,
    pub last_price: f64,
    pub volume: u32,
    pub average_price: f64,
    pub oi: f64,
    pub net_change: f64,
    pub total_buy_quantity: f64,
    pub total_sell_quantity: f64,
    pub lower_circuit_limit: f64,
    pub upper_circuit_limit: f64,
    pub last_trade_time: String,
    pub oi_day_high: f64,
    pub oi_day_low: f64,
}
