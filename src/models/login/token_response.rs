use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenResponse {
    pub email: String,
    pub exchanges: Vec<String>,
    pub products: Vec<String>,
    pub broker: String,
    pub user_id: String,
    pub user_name: String,
    pub order_types: Vec<String>,
    pub user_type: String,
    pub poa: bool,
    pub is_active: bool,
    pub access_token: String,
    pub extended_token: Option<String>,
}
