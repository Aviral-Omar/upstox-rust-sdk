use {
    crate::models::{market_information::market_timings_response::MarketTimingResponse, Exchange},
    serde::Deserialize,
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HolidayType {
    SettlementHoliday,
    TradingHoliday,
    SpecialTiming,
}

#[derive(Deserialize, Debug)]
pub struct MarketHolidayResponse {
    pub date: String,
    pub description: String,
    pub holiday_type: HolidayType,
    pub closed_exchanges: Vec<Exchange>,
    pub open_exchanges: Vec<MarketTimingResponse>,
}
