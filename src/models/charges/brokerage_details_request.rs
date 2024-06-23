use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BrokerageDetailsRequest {
    pub instrument_token: String,
    pub quantity: u32,
    pub product: String,
    pub transaction_type: String,
    pub price: f64,
}
