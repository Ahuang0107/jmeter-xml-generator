use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::elements::root;
use crate::xml::EventWriter;

#[cfg(test)]
mod test;

#[wasm_bindgen]
pub struct ScriptBuilder {
    variables: std::collections::HashMap<String, String>,
    headers: std::collections::HashMap<String, String>,
    requests: Vec<RequestArg>,
    timer: instant::Instant,
}

#[derive(Clone)]
pub struct RequestArg {
    pub protocol: String,
    pub host: String,
    pub port: String,
    pub method: String,
    pub path: String,
    pub heads: std::collections::HashMap<String, String>,
    pub params: std::collections::HashMap<String, String>,
    pub data: serde_json::Value,
    pub delay_time: u128,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxiosConfigRaw {
    base_url: Option<String>,
    url: Option<String>,
    method: Option<String>,
    heads: Option<serde_json::Value>,
    params: Option<serde_json::Value>,
    data: Option<serde_json::Value>,
}

#[allow(dead_code)]
pub struct AxiosConfig {
    base_url: String,
    url: String,
    method: String,
    heads: std::collections::HashMap<String, String>,
    params: std::collections::HashMap<String, String>,
    data: serde_json::Value,
}

#[allow(dead_code)]
impl AxiosConfigRaw {
    pub fn from_str(str: &str) -> AxiosConfigRaw {
        serde_json::from_str::<AxiosConfigRaw>(str).unwrap_or_else(|_| {
            if cfg!(target_arch = "wasm32") {
                log::error!("unable to serialize {:?}", str);
            }
            panic!("unable to serialize {:?}", str);
        })
    }
    pub fn into_axios_config(self) -> AxiosConfig {
        let real_heads = match self.heads {
            Some(v) => serialize_into_string_map_from_value(v),
            None => std::collections::HashMap::new(),
        };
        let real_params = match self.params {
            Some(v) => serialize_into_string_map_from_value(v),
            None => std::collections::HashMap::new(),
        };
        AxiosConfig {
            base_url: self.base_url.unwrap_or("".to_string()),
            url: self.url.unwrap_or("".to_string()),
            method: self.method.unwrap_or("".to_string()),
            heads: real_heads,
            params: real_params,
            data: self.data.unwrap_or(serde_json::Value::default()),
        }
    }
}

#[allow(dead_code)]
impl AxiosConfig {
    pub fn into_request_arg(mut self, delay_time: u128) -> RequestArg {
        let actual_url = self.base_url + &*self.url;
        let split = actual_url.split('?').collect::<Vec<&str>>();
        let actual_url = split.first().unwrap_or(&"");
        let path_params_string = split.last().unwrap_or(&"");
        let path_params = path_params_string.split('&').collect::<Vec<&str>>();
        path_params.iter().for_each(|s| {
            let kv = s.split('=').collect::<Vec<&str>>();
            let k = kv.first().unwrap_or(&"");
            let v = kv.last().unwrap_or(&"");
            self.params.insert(k.to_string(), v.to_string());
        });
        let url = url::Url::parse(&*actual_url).unwrap_or_else(|_| {
            if cfg!(target_arch = "wasm32") {
                log::error!("unable to parse url {:?}", actual_url);
            }
            panic!("unable to parse url {:?}", actual_url);
        });
        let protocol = if url.scheme() == "http" {
            "http".to_string()
        } else if url.scheme() == "https" {
            "https".to_string()
        } else {
            if cfg!(target_arch = "wasm32") {
                log::error!("unable to find request url protocol {:?}", url);
            }
            panic!("unable to find request url protocol {:?}", url);
        };
        let host = if let Some(host_str) = url.host_str() {
            host_str.to_string()
        } else {
            if cfg!(target_arch = "wasm32") {
                log::error!("unable to find request url host {:?}", url);
            }
            panic!("unable to find request url host {:?}", url);
        };
        let port = if let Some(p) = url.port() {
            p.to_string()
        } else {
            "".to_string()
        };
        let path = url.path().replace("//", "/");
        RequestArg {
            protocol,
            host,
            port,
            method: self.method.to_uppercase(),
            path,
            heads: self.heads,
            params: self.params,
            data: self.data,
            delay_time,
        }
    }
}

#[allow(dead_code)]
pub fn serialize_into_string_map_from_str(str: &str) -> std::collections::HashMap<String, String> {
    return match serde_json::from_str::<serde_json::Value>(str) {
        Ok(value) => serialize_into_string_map_from_value(value),
        Err(_) => {
            if cfg!(target_arch = "wasm32") {
                log::error!("unable to serialize {:?}", str);
            }
            panic!("unable to serialize {:?}", str);
        }
    };
}

#[allow(dead_code)]
pub fn serialize_into_string_map_from_value(
    value: serde_json::Value,
) -> std::collections::HashMap<String, String> {
    let map: std::collections::HashMap<String, String> = if let Some(o) = value.as_object() {
        o.into_iter()
            .map(|(k, v)| {
                let string = if let Some(str) = v.as_str() {
                    str.to_string()
                } else {
                    v.to_string()
                };
                return (k.clone(), string);
            })
            .collect()
    } else {
        std::collections::HashMap::new()
    };
    map
}

#[allow(dead_code)]
#[wasm_bindgen]
impl ScriptBuilder {
    /// create a script builder that can generate a jmeter test plan script
    pub fn new() -> ScriptBuilder {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        wasm_logger::init(wasm_logger::Config::default());
        ScriptBuilder {
            variables: std::collections::HashMap::new(),
            headers: std::collections::HashMap::new(),
            requests: Vec::new(),
            timer: instant::Instant::now(),
        }
    }
    pub fn add_header(&mut self, k: String, v: String) {
        self.headers.insert(k, v);
    }
    /// add axios request
    ///
    /// * `axios_config_raw` - serialise of AxiosConfigRaw
    pub fn add_axios_request(&mut self, axios_config_raw: String) {
        let last_time = self.timer;
        self.timer = instant::Instant::now();
        self.requests.push(
            AxiosConfigRaw::from_str(axios_config_raw.as_str())
                .into_axios_config()
                .into_request_arg(self.timer.duration_since(last_time).as_millis()),
        );
        if cfg!(target_arch = "wasm32") {
            log::info!("succeed add request")
        }
    }
    /// generate the test plan script
    /// return file data
    pub fn build(&self) -> Vec<u8> {
        let target: Vec<u8> = Vec::new();
        let mut writer = EventWriter::new(target);
        let script = root(
            self.variables
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect::<std::collections::HashMap<&str, &str>>(),
            self.headers
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect::<Vec<(&str, &str)>>(),
            self.requests
                .iter()
                .map(|r| r.clone())
                .collect::<Vec<RequestArg>>(),
        );

        script.write(&mut writer);
        writer.export()
    }
}
