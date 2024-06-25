pub mod charges;
pub mod error_response;
pub mod historical_data;
pub mod login;
pub mod market_information;
pub mod market_quote;
pub mod option_chain;
pub mod orders;
pub mod portfolio;
pub mod success_response;
pub mod trade_profit_and_loss;
pub mod user;

use {
    crate::utils::serde_spaced_lowercase,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
    std::{fmt, str::FromStr},
};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ProductType {
    I,
    D,
    CO,
    MTF,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TransactionType {
    Buy,
    Sell,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SegmentType {
    EQ,
    FO,
    COM,
    CD,
    MF,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Exchange {
    NSE,
    NFO,
    CDS,
    BSE,
    BFO,
    BCD,
    MCX,
}

#[derive(Debug)]
pub enum OrderStatus {
    ValidationPending,
    ModifyPending,
    TriggerPending,
    PutOrderReqReceived,
    ModifyAfterMarketOrderReqReceived,
    CancelledAfterMarketOrder,
    Open,
    Complete,
    ModifyValidationPending,
    AfterMarketOrderReqReceived,
    Modified,
    NotCancelled,
    CancelPending,
    Rejected,
    Cancelled,
    OpenPending,
    NotModified,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            OrderStatus::ValidationPending => "ValidationPending",
            OrderStatus::ModifyPending => "ModifyPending",
            OrderStatus::TriggerPending => "TriggerPending",
            OrderStatus::PutOrderReqReceived => "PutOrderReqReceived",
            OrderStatus::ModifyAfterMarketOrderReqReceived => "ModifyAfterMarketOrderReqReceived",
            OrderStatus::CancelledAfterMarketOrder => "CancelledAfterMarketOrder",
            OrderStatus::Open => "Open",
            OrderStatus::Complete => "Complete",
            OrderStatus::ModifyValidationPending => "ModifyValidationPending",
            OrderStatus::AfterMarketOrderReqReceived => "AfterMarketOrderReqReceived",
            OrderStatus::Modified => "Modified",
            OrderStatus::NotCancelled => "NotCancelled",
            OrderStatus::CancelPending => "CancelPending",
            OrderStatus::Rejected => "Rejected",
            OrderStatus::Cancelled => "Cancelled",
            OrderStatus::OpenPending => "OpenPending",
            OrderStatus::NotModified => "NotModified",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for OrderStatus {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "validation pending" => Ok(OrderStatus::ValidationPending),
            "modify pending" => Ok(OrderStatus::ModifyPending),
            "trigger pending" => Ok(OrderStatus::TriggerPending),
            "put order req received" => Ok(OrderStatus::PutOrderReqReceived),
            "modify after market order req received" => {
                Ok(OrderStatus::ModifyAfterMarketOrderReqReceived)
            }
            "cancelled after market order" => Ok(OrderStatus::CancelledAfterMarketOrder),
            "open" => Ok(OrderStatus::Open),
            "complete" => Ok(OrderStatus::Complete),
            "modify validation pending" => Ok(OrderStatus::ModifyValidationPending),
            "after market order req received" => Ok(OrderStatus::AfterMarketOrderReqReceived),
            "modified" => Ok(OrderStatus::Modified),
            "not cancelled" => Ok(OrderStatus::NotCancelled),
            "cancel pending" => Ok(OrderStatus::CancelPending),
            "rejected" => Ok(OrderStatus::Rejected),
            "cancelled" => Ok(OrderStatus::Cancelled),
            "open pending" => Ok(OrderStatus::OpenPending),
            "not modified" => Ok(OrderStatus::NotModified),
            _ => Err("Invalid order status"),
        }
    }
}

impl Serialize for OrderStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serde_spaced_lowercase::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for OrderStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        serde_spaced_lowercase::deserialize(deserializer)
    }
}
