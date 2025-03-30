use {
    crate::models::{
        gtt_orders::{validate_product_type, GTTOrderRule, GTTOrderType},
        ProductType, TransactionType,
    },
    serde::Serialize,
    serde_valid::Validate,
};

#[derive(Serialize, Debug, Validate)]
pub struct PlaceGTTOrderRequest {
    #[serde(rename = "type")]
    pub gtt_order_type: GTTOrderType,
    #[validate(exclusive_minimum = 0, message = "quantity must be greater than 0")]
    pub quantity: u32,
    #[validate(custom(validate_product_type))]
    pub product: ProductType,
    #[validate(
        pattern = r"^(?:^NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+(,(?:NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+)*?$",
        message = "Invalid instrument_token"
    )]
    pub instrument_token: String,
    pub transaction_type: TransactionType,
    pub rules: Vec<GTTOrderRule>,
}
