use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    Code,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DialogRequest {
    pub client_id: String,
    pub redirect_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    pub response_type: ResponseType,
}
