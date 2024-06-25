use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ConvertPositionsResponse {
    pub status: String,
}
