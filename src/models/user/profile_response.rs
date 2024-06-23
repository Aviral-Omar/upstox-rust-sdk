use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProfileResponse {
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
}
