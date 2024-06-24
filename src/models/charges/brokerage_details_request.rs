use {
    crate::models::{ProductType, TransactionType},
    serde::Serialize,
    serde_valid::Validate,
};

#[derive(Serialize, Debug, Validate)]
pub struct BrokerageDetailsRequest {
    #[validate(
        pattern = r"^(?:^NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+(,(?:NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+)*?$",
        message = "Invalid instrument_token"
    )]
    pub instrument_token: String,
    #[validate(exclusive_minimum = 0, message = "quantity must be greater than 0")]
    pub quantity: u32,
    pub product: ProductType,
    pub transaction_type: TransactionType,
    #[validate(exclusive_minimum = 0.0, message = "price must be greater than 0.0")]
    pub price: f64,
}
