use {
    crate::models::{ProductType, TransactionType},
    serde::Serialize,
    serde_valid::Validate,
};

#[derive(Serialize, Debug, Validate)]
pub struct Instrument {
    #[validate(
        pattern = r"^(?:^NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+(,(?:NSE_EQ|NSE_FO|NCD_FO|BSE_EQ|BSE_FO|BCD_FO|MCX_FO|NSE_INDEX|BSE_INDEX|MCX_INDEX)\|[\w ]+)*?$",
        message = "Invalid instrument_key"
    )]
    pub instrument_key: String,
    #[validate(exclusive_minimum = 0, message = "quantity must be greater than 0")]
    pub quantity: u32,
    pub product: ProductType,
    pub transaction_type: TransactionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}

#[derive(Serialize, Debug, Validate)]
pub struct MarginDetailsRequest {
    pub instruments: Vec<Instrument>,
}
