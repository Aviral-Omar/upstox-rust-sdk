use serde::{Deserialize, Deserializer, Serializer};
use std::fmt;
use std::str::FromStr;

fn to_spaced_lower_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i != 0 {
            result.push(' ');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: fmt::Display,
{
    let s = value.to_string();
    let spaced_lower = to_spaced_lower_case(&s);
    serializer.serialize_str(&spaced_lower)
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr<Err = &'static str>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse().map_err(|err| serde::de::Error::custom(err))
}
