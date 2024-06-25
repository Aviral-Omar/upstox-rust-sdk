use {
    crate::models::ProductType,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct HoldingsResponse {
    pub isin: String,
    pub cnc_used_quantity: u32,
    pub collateral_type: String,
    pub company_name: String,
    pub haircut: f64,
    pub product: ProductType,
    pub quantity: u32,
    pub trading_symbol: String,
    pub last_price: f64,
    pub pnl: f64,
    pub day_change: f64,
    pub day_change_percentage: f64,
    pub instrument_token: String,
    pub average_price: f64,
    pub collateral_quantity: u32,
    pub collateral_update_quantity: u32,
    pub t1_quantity: u32,
    pub exchange: String,
}
