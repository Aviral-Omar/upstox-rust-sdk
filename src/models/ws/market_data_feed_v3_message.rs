use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MethodTypeV3 {
    Sub,
    ChangeMode,
    Unsub,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ModeTypeV3 {
    LTPC,
    OptionGreeks,
    Full,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MarketDataFeedV3Message {
    pub guid: String,
    pub method: MethodTypeV3,
    pub data: MessageDataV3,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageDataV3 {
    pub mode: ModeTypeV3,
    #[serde(rename = "instrumentKeys")]
    pub instrument_keys: Vec<String>,
}
