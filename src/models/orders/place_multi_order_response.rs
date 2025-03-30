use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CorrelatedOrder {
    pub correlation_id: String,
    pub order_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PlaceMultiOrderResponse {
    pub order_ids: Vec<CorrelatedOrder>,
}
