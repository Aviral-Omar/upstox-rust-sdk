use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OrdersResponse {
    pub order_ids: Vec<String>,
}
