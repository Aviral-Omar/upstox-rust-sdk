use {serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct OrderDetailsRequest {
    // One of order_id or tag has to be given
    #[validate(pattern = r"^[-a-zA-Z0-9]+", message = "Invalid order_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}
