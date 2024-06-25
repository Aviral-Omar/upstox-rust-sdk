use {crate::models::Exchange, serde::Deserialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketStatus {
    NormalOpen,
    NormalClose,
    PreOpenStart,
    PreOpenEnd,
    ClosingStart,
    ClosingEnd,
}

#[derive(Deserialize, Debug)]
pub struct ExchangeStatusResponse {
    pub exchange: Exchange,
    pub status: MarketStatus,
    pub last_updated: u64,
}
