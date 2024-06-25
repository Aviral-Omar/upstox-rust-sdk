use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TradeType {
    FUT,
    OPT,
    EQ,
}

#[derive(Deserialize, Debug)]
pub struct ProfitAndLossResponse {
    pub quantity: f64,
    pub isin: String,
    pub scrip_name: String,
    pub trade_type: TradeType,
    pub buy_date: String,
    pub buy_average: f64,
    pub sell_date: String,
    pub sell_average: f64,
    pub buy_amount: f64,
    pub sell_amount: f64,
}
