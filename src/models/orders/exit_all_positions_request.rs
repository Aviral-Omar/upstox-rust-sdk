use {crate::models::orders::ExchangeSegment, serde::Serialize, serde_valid::Validate};

#[derive(Serialize, Debug, Validate)]
pub struct ExitAllPositionsRequest {
    pub segment: Option<ExchangeSegment>,
    pub taget: Option<String>,
}
