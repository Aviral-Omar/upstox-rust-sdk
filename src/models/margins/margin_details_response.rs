use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct MarginsData {
    pub equity_margin: f64,
    pub total_margin: f64,
    pub exposure_margin: f64,
    pub tender_margin: f64,
    pub span_margin: f64,
    pub net_buy_premium_margin: f64,
    pub additional_margin: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MarginDetailsResponse {
    pub required_margin: f64,
    pub final_margin: f64,
    pub margins: Vec<MarginsData>,
}
