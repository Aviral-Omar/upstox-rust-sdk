use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConvertPositionsResponse {
    pub status: String,
}
