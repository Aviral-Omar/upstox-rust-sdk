use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Taxes {
    pub gst: f64,
    pub stt: f64,
    pub stamp_duty: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OtherCharges {
    pub transaction: f64,
    pub clearing: f64,
    pub ipft: f64,
    pub sebi_turnover: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DepositoryParticipantPlan {
    pub name: String,
    pub min_expensse: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ChargesData {
    pub total: f64,
    pub brokerage: f64,
    pub taxes: Taxes,
    pub other_charges: OtherCharges,
    pub dp_plan: Option<DepositoryParticipantPlan>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BrokerageDetailsResponse {
    pub charges: ChargesData,
}
