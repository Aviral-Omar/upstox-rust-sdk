pub mod cancel_multi_order_request;
pub mod cancel_multi_order_response;
pub mod cancel_order_request;
pub mod exit_all_positions_request;
pub mod modify_order_request;
pub mod order_details_request;
pub mod order_details_response;
pub mod order_response;
pub mod order_trades_request;
pub mod orders_response;
pub mod place_multi_order_request;
pub mod place_multi_order_response;
pub mod place_order_request;
pub mod place_order_v3_request;
pub mod trade_details_response;
pub mod trade_history_request;
pub mod trade_history_response;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ValidityType {
    DAY,
    IOC,
}

#[derive(Serialize, Debug, Eq, Hash, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExchangeSegment {
    NseEq,
    NseFo,
    NseCom,
    NcdFo,
    BseEq,
    BseFo,
    BcdFo,
    McxFo,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum OrderType {
    Market,
    Limit,
    SL,
    SlM,
}
