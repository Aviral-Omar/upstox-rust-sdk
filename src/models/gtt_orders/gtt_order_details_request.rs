use {serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct GTTOrderDetailsRequest {
    #[validate(pattern = r"^GTT-[a-zA-Z0-9]+$", message = "Invalid gtt_order_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtt_order_id: Option<String>,
}
