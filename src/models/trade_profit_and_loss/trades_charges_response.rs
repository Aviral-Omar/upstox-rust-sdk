use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Taxes {
    pub gst: f64,
    pub stt: f64,
    pub stamp_duty: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Charges {
    pub transaction: f64,
    pub clearing: f64,
    pub ipft: Option<f64>,
    pub others: f64,
    pub sebi_turnover: f64,
    pub demat_transaction: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChargesBreakdown {
    pub total: f64,
    pub brokerage: f64,
    pub taxes: Taxes,
    pub charges: Charges,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TradesChargesResponse {
    pub charges_breakdown: ChargesBreakdown,
}
