use {
    crate::models::{AssetType, Exchange, ExchangeSegment},
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum InstrumentsResponse {
    EquityResponse {
        segment: ExchangeSegment,
        name: String,
        exchange: Exchange,
        isin: String,
        instrument_type: String,
        instrument_key: String,
        lot_size: u32,
        freeze_quantity: f64,
        exchange_token: String,
        tick_size: f64,
        trading_symbol: String,
        short_name: Option<String>,
        security_type: Option<String>,
        qty_multiplier: Option<f64>,
    },
    DerivativeResponse {
        weekly: Option<bool>,
        segment: ExchangeSegment,
        name: String,
        exchange: Exchange,
        expiry: u64,
        instrument_type: String,
        asset_symbol: String,
        underlying_symbol: String,
        instrument_key: String,
        lot_size: u32,
        freeze_quantity: f64,
        exchange_token: String,
        minimum_lot: Option<u32>,
        asset_key: Option<String>,
        underlying_key: Option<String>,
        tick_size: f64,
        asset_type: AssetType,
        underlying_type: AssetType,
        trading_symbol: String,
        strike_price: f64,
        last_trading_date: Option<u64>,
        price_quote_unit: Option<String>,
        qty_multiplier: Option<f64>,
    },
    IndexResponse {
        segment: ExchangeSegment,
        name: String,
        exchange: Exchange,
        instrument_type: String,
        instrument_key: String,
        exchange_token: Option<String>,
        trading_symbol: String,
    },
    CommodityResponse {
        segment: ExchangeSegment,
        name: String,
        exchange: Exchange,
        instrument_type: String,
        instrument_key: String,
        lot_size: u32,
        freeze_quantity: f64,
        exchange_token: String,
        tick_size: f64,
        trading_symbol: String,
        strike_price: f64,
        price_quote_unit: Option<String>,
        qty_multiplier: f64,
    },
}
