use {serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct OrderTradesRequest {
    #[validate(pattern = r"^[-a-zA-Z0-9]+", message = "Invalid order_id")]
    pub order_id: String,
}
