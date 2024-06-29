use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MethodType {
    Sub,
    ChangeMode,
    Unsub,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ModeType {
    LTPC,
    OptionChain,
    Full,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MarketDataFeedMessage {
    pub guid: String,
    pub method: MethodType,
    pub data: MessageData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageData {
    pub mode: String,
    #[serde(alias = "instrumentKeys")]
    pub instrument_keys: MethodType,
}
