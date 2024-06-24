use {
    crate::models::{SegmentType, TransactionType},
    serde::Deserialize,
};

#[derive(Deserialize, Debug)]
pub struct TradeHistoryResponse {
    pub exchange: String,
    pub segment: SegmentType,
    pub option_type: String,
    pub quantity: u32,
    pub amount: f64,
    pub trade_id: String,
    pub trade_date: String,
    pub transaction_type: TransactionType,
    pub scrip_name: String,
    pub strike_price: String,
    pub expiry: String,
    pub price: f64,
    pub isin: String,
    pub symbol: String,
    pub instrument_token: String,
}
