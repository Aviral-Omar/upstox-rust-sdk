use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct OrderResponse {
    pub order_id: String,
}
