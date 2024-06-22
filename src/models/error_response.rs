use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Errors {
    pub error_code: String,
    pub message: String,
    pub property_path: Option<String>,
    pub invalid_value: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub errors: Vec<Errors>,
}
