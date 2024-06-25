use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SegmentFundAndMarginData {
    pub used_margin: f64,
    pub payin_amount: f64,
    pub span_margin: f64,
    pub adhoc_margin: f64,
    pub notional_cash: f64,
    pub available_margin: f64,
    pub exposure_margin: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FundAndMarginResponse {
    pub commodity: Option<SegmentFundAndMarginData>,
    pub equity: Option<SegmentFundAndMarginData>,
}
