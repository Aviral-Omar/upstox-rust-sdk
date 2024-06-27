use {
    crate::models::{AssetType, ExchangeSegment},
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum InstrumentType {
    CE,
    PE,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OptionContractResponse {
    pub name: String,
    pub segment: ExchangeSegment,
    pub expiry: String,
    pub instrument_key: String,
    pub exchange_token: String,
    pub trading_symbol: String,
    pub tick_size: f64,
    pub lot_size: f64,
    pub instrument_type: InstrumentType,
    pub freeze_quantity: f64,
    pub underlying_key: String,
    pub underlying_type: AssetType,
    pub underlying_symbol: String,
    pub strike_price: f64,
    pub minimum_lot: f64,
    pub weekly: bool,
}
