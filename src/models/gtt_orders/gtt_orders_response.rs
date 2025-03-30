use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GTTOrdersResponse {
    pub gtt_order_ids: Vec<String>,
}
