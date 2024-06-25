use {crate::models::Exchange, serde::Serialize};

#[derive(Serialize, Debug)]
pub struct ExchangeStatusRequest {
    pub exchange: Exchange,
}
