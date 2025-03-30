use {
    crate::models::{
        gtt_orders::{GTTOrderTriggerType, GTTOrderType, GTTRuleStatus, GTTRuleStrategy},
        ProductType, TransactionType,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct GTTOrderDetailsRule {
    pub strategy: GTTRuleStrategy,
    pub status: GTTRuleStatus,
    pub trigger_type: GTTOrderTriggerType,
    pub trigger_price: f64,
    pub transaction_type: TransactionType,
    pub message: String,
    pub order_id: Option<String>,
}

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
