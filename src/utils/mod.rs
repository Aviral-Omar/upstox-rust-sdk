pub mod serde_spaced_lowercase;

use {
    crate::constants::{APIVersion, BaseUrlType},
    serde::Serialize,
    serde_json::Value,
    std::{
        fs::File,
        io::{Read, Write},
    },
};

pub trait ToKeyValueTuples: Send {
    fn to_key_value_tuples_vec(&self) -> Vec<(String, String)>;
}

impl<T: Serialize + Send> ToKeyValueTuples for T {
    fn to_key_value_tuples_vec(&self) -> Vec<(String, String)> {
        let value: Value = serde_json::to_value(self).expect("Failed to serialize");
        match value {
            Value::Object(map) => map
                .into_iter()
                .filter_map(|(k, v)| match v {
                    Value::String(s) => Some((k, s)),
                    Value::Number(n) => Some((k, n.to_string())),
                    Value::Bool(b) => Some((k, b.to_string())),
                    Value::Null => None,
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }
}

pub fn write_value_to_file(filename: &str, value: &str) -> std::io::Result<()> {
    let mut file: File = File::create(filename)?;
    file.write_all(value.as_bytes())?;
    Ok(())
}

pub fn read_value_from_file(filename: &str) -> std::io::Result<String> {
    let mut file: File = File::open(filename)?;
    let mut value: String = String::new();
    file.read_to_string(&mut value)?;
    Ok(value.trim().to_string())
}

pub fn create_url(base_url_type: BaseUrlType, api_version: APIVersion, endpoint: &str) -> String {
    format!(
        "https://api{}.upstox.com/{}{}",
        match base_url_type {
            BaseUrlType::REGULAR => "",
            BaseUrlType::HFT => "-hft",
            BaseUrlType::SANDBOX => "-sandbox",
        },
        match api_version {
            APIVersion::V2 => "v2",
            APIVersion::V3 => "v3",
        },
        endpoint
    )
}
