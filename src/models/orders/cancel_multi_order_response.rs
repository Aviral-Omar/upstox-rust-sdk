use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CancelMultiOrderResponse {
    pub order_ids: Vec<String>,
}
