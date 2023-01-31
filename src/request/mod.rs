use crate::axios_config::AxiosRequestConfig;

/// 添加一个HTTP Request所需要的参数
#[derive(Clone, Debug)]
pub struct RequestArg {
    /// GET or POST or PUT
    method: String,
    /// http or https
    protocol: String,
    /// github.com or 20.205.243.166
    host: String,
    port: String,
    path: String,
    headers: Vec<(String, String)>,
    param: serde_json::Value,
    delay: u128,
}

impl RequestArg {
    pub fn new() -> Self {
        Self {
            method: String::new(),
            protocol: String::new(),
            host: String::new(),
            port: String::new(),
            path: String::new(),
            headers: vec![],
            param: serde_json::Value::Null,
            delay: 0,
        }
    }
    pub fn set_method(&mut self, method: &str) {
        self.method = method.to_string();
    }
    pub fn set_protocol(&mut self, protocol: &str) {
        self.protocol = protocol.to_string();
    }
    pub fn set_host(&mut self, host: &str) {
        self.host = host.to_string();
    }
    pub fn set_port(&mut self, port: &str) {
        self.port = port.to_string();
    }
    pub fn set_path(&mut self, path: &str) {
        self.path = path.to_string();
    }
    pub fn set_header(&mut self, header: (&str, &str)) {
        self.headers
            .push((header.0.to_string(), header.1.to_string()));
    }
    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }
    pub fn set_param(&mut self, param: serde_json::Value) {
        self.param = param;
    }
    pub fn set_delay(&mut self, delay: u128) {
        self.delay = delay;
    }
    pub fn method(&self) -> &str {
        self.method.as_str()
    }
    pub fn protocol(&self) -> &str {
        self.protocol.as_str()
    }
    pub fn host(&self) -> &str {
        self.host.as_str()
    }
    pub fn port(&self) -> &str {
        self.port.as_str()
    }
    pub fn path(&self) -> &str {
        self.path.as_str()
    }
    pub fn headers(&self) -> &Vec<(String, String)> {
        &self.headers
    }
    pub fn delay(&self) -> u128 {
        self.delay
    }
    pub fn if_form_data_body(&self) -> bool {
        self.method == "POST"
            && self.headers.contains(&(
                String::from("Content-Type"),
                String::from("multipart/form-data"),
            ))
    }
    pub fn if_json_body(&self) -> bool {
        (self.method == "POST" || self.method == "PUT") && !self.if_form_data_body()
    }
    pub fn body(&self) -> serde_json::Value {
        self.param.clone()
    }
    pub fn body_string(&self) -> String {
        serde_json::to_string(&self.param).unwrap()
    }
    pub fn params(&self) -> Vec<(String, String)> {
        let mut result = vec![];
        if let Some(map) = self.param.as_object() {
            map.iter().for_each(|(k, v)| {
                if let Some(v) = v.as_str() {
                    result.push((k.clone(), String::from(v)))
                } else if let Some(v) = v.as_bool() {
                    result.push((k.clone(), v.to_string()))
                } else if let Some(v) = v.as_u64() {
                    result.push((k.clone(), v.to_string()))
                } else if let Some(v) = v.as_f64() {
                    result.push((k.clone(), v.to_string()))
                }
            })
        }
        result
    }
}

impl From<AxiosRequestConfig> for RequestArg {
    fn from(value: AxiosRequestConfig) -> Self {
        let url = value.base_url + &value.url;
        let url = url::Url::parse(&url).unwrap();

        let mut result = RequestArg::new();
        result.set_method(&value.method);
        result.set_protocol(url.scheme());
        result.set_host(url.host_str().unwrap());
        if let Some(port) = url.port() {
            result.set_port(&port.to_string());
        }
        let path = url.path();
        result.set_path(url.path());
        if let Some(headers) = value.headers.as_object() {
            headers.iter().for_each(|(k, v)| {
                if let Some(v) = v.as_str() {
                    result.set_header((k, v))
                }
            });
        }
        result.set_param(value.params);
        result
    }
}

#[cfg(test)]
mod test;
