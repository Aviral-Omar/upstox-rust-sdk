use {
    crate::models::ResponseSummary,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Errors {
    pub error_code: String,
    pub message: String,
    pub property_path: Option<String>,
    pub invalid_value: Option<String>,
    pub order_id: Option<String>,
    pub instrument_key: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub data: Option<()>,
    pub errors: Vec<Errors>,
    pub summary: Option<ResponseSummary>,
}
