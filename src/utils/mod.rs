pub mod serde_spaced_lowercase;

use std::{
    fs::File,
    io::{Read, Write},
};

use serde::Serialize;
use serde_json::Value;

pub trait ToKeyValueTuples {
    fn to_key_value_tuples_vec(&self) -> Vec<(String, String)>;
}

impl<T: Serialize> ToKeyValueTuples for T {
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
    Ok(value)
}
