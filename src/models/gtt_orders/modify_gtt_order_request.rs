use {
    crate::models::gtt_orders::{GTTOrderRule, GTTOrderType},
    serde::Serialize,
    serde_valid::Validate,
};

#[derive(Serialize, Debug, Validate)]
pub struct ModifyGTTOrderRequest {
    #[serde(rename = "type")]
    pub gtt_order_type: GTTOrderType,
    #[validate(exclusive_minimum = 0, message = "quantity must be greater than 0")]
    pub quantity: u32,
    #[validate(pattern = r"^GTT-[a-zA-Z0-9]+$", message = "Invalid gtt_order_id")]
    pub gtt_order_id: String,
    pub rules: Vec<GTTOrderRule>,
}
