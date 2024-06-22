use std::{fmt::Display, str::FromStr};

use serde::{de, Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
pub struct SuccessResponse<T> {
    pub status: String,
    #[serde(deserialize_with = "deserialize_from_str")]
    #[serde(bound(deserialize = "T: FromStr, T::Err: Display"))]
    pub data: T,
}

fn deserialize_from_str<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}
