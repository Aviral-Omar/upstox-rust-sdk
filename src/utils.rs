use serde::Serialize;
use serde_json::Value;

pub trait ToQueryParams {
    fn to_query_params(&self) -> Vec<(String, String)>;
}

impl<T: Serialize> ToQueryParams for T {
    fn to_query_params(&self) -> Vec<(String, String)> {
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
