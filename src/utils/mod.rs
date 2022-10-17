use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    #[allow(dead_code)]
    pub fn from(key: String, value: String) -> KeyValue {
        KeyValue { key, value }
    }
}
