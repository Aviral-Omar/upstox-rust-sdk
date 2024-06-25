use {crate::models::Exchange, serde::Deserialize};

#[derive(Deserialize, Debug)]
pub struct MarketTimingResponse {
    pub exchange: Exchange,
    pub start_time: u64,
    pub end_time: u64,
}
