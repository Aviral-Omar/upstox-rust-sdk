use {
    crate::models::{
        ProductType,
        gtt_orders::{GTTOrderDetailsRule, GTTOrderType},
    },
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct GTTOrderDetailsResponse {
    #[serde(rename = "type")]
    pub gtt_order_type: GTTOrderType,
    pub exchange: String,
    pub quantity: u32,
    pub product: ProductType,
    pub instrument_token: String,
    pub trading_symbol: String,
    pub gtt_order_id: String,
    pub expires_at: u64,
    pub created_at: u64,
    pub rules: Vec<GTTOrderDetailsRule>,
}
