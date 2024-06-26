use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum TokenType {
    Bearer,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GoogleOAuth2TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub id_token: Option<String>,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: TokenType,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GoogleOAuth2TokenErrorResponse {
    pub error: String,
    pub error_description: String,
}
