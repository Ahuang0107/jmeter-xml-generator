#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct AxiosRequestConfig {
    pub method: String,
    #[serde(rename = "baseURL")]
    pub base_url: String,
    pub url: String,
    #[serde(default)]
    pub headers: serde_json::Value,
    #[serde(default)]
    pub params: serde_json::Value,
    #[serde(default)]
    pub data: serde_json::Value,
}

impl AxiosRequestConfig {
    #[cfg(target_arch = "wasm32")]
    pub fn from(val: wasm_bindgen::JsValue) -> Self {
        serde_wasm_bindgen::from_value(val).unwrap()
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn from(val: &str) -> Self {
        serde_json::from_str(val).unwrap()
    }
}

#[cfg(test)]
mod test;
