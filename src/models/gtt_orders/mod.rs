pub mod cancel_gtt_order_request;
pub mod gtt_order_details_request;
pub mod gtt_order_details_response;
pub mod gtt_orders_response;
pub mod modify_gtt_order_request;
pub mod place_gtt_order_request;

use {
    crate::models::ProductType,
    serde::{Deserialize, Serialize},
    serde_valid::{validation::Error, Validate},
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum GTTOrderType {
    Single,
    Multiple,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum GTTRuleStrategy {
    Entry,
    Target,
    StopLoss,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum GTTRuleStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum GTTOrderTriggerType {
    Below,
    Above,
    Immediate,
}

#[derive(Serialize, Debug, Validate)]
pub struct GTTOrderRule {
    pub strategy: GTTRuleStrategy,
    pub trigger_type: GTTOrderTriggerType,
    #[validate(
        exclusive_minimum = 0.0,
        message = "trigger_price must be greater than 0.0"
    )]
    pub trigger_price: f64,
}

pub(super) fn validate_product_type(product: &ProductType) -> Result<(), Error> {
    match product {
        ProductType::I | ProductType::D => Ok(()),
        _ => Err(Error::Custom("product_type must be I or D".to_string())),
    }
}
