use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
}

#[derive(Serialize, Debug)]
pub struct TokenRequest {
    pub code: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub grant_type: GrantType,
}
