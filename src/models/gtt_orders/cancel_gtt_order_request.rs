use {serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct CancelGTTOrderRequest {
    #[validate(pattern = r"^GTT-[a-zA-Z0-9]+$", message = "Invalid gtt_order_id")]
    pub gtt_order_id: String,
}
