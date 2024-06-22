use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Code,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AccessType {
    Offline,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Prompt {
    Consent,
    SelectAccount,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
}

#[derive(Serialize, Debug)]
pub enum CodeChallengeMethod {
    S256,
    #[serde(rename = "plain")]
    Plain,
}

#[derive(Serialize, Debug)]
pub struct GoogleOAuth2AuthRequest {
    pub client_id: String,
    pub redirect_uri: String,
    pub response_type: ResponseType,
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_challenge_method: Option<CodeChallengeMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<AccessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Prompt>,
}

#[derive(Serialize, Debug)]
pub struct GoogleOAuth2CodeTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verifier: Option<String>,
    pub grant_type: GrantType,
    pub redirect_uri: String,
}

#[derive(Serialize, Debug)]
pub struct GoogleOAuth2RefreshTokenRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: GrantType,
    pub refresh_token: String,
}
