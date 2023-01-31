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
    // Display Name
    pub(crate) name: Option<String>,
    pub(crate) json_post_processors: Vec<(String, String)>,
    pub(crate) groovy_pre_processors: Vec<String>,
    pub(crate) groovy_post_processors: Vec<String>,
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
            name: None,
            json_post_processors: vec![],
            groovy_pre_processors: vec![],
            groovy_post_processors: vec![],
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
    pub fn add_json_processor(&mut self, value: (&str, &str)) {
        self.json_post_processors
            .push((value.0.to_string(), value.1.to_string()))
    }
    pub fn with_json_processor(mut self, value: (&str, &str)) -> Self {
        self.add_json_processor(value);
        self
    }
    fn add_groovy_processor(&mut self, value: &str, post: bool) {
        if post {
            self.groovy_post_processors.push(value.to_string());
        } else {
            self.groovy_pre_processors.push(value.to_string());
        }
    }
    pub fn with_groovy_processor(mut self, value: &str, post: bool) -> Self {
        self.add_groovy_processor(value, post);
        self
    }
    pub fn set_delay(&mut self, delay: u128) {
        self.delay = delay;
    }
    pub fn with_delay(mut self, delay: u128) -> Self {
        self.delay = delay;
        self
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
    pub fn if_form_body(&self) -> bool {
        self.method == "POST"
            && self.headers.contains(&(
                String::from("Content-Type"),
                String::from("application/x-www-form-urlencoded"),
            ))
    }
    pub fn if_json_body(&self) -> bool {
        (self.method == "POST" || self.method == "PUT") && !self.if_form_body()
    }
    #[cfg(test)]
    pub fn body(&self) -> serde_json::Value {
        self.param.clone()
    }
    pub fn body_string(&self) -> String {
        remove_json_string_variable_quota(serde_json::to_string(&self.param).unwrap())
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
    pub fn from_get(url: &str, param: Option<serde_json::Value>) -> Self {
        let mut arg = RequestArg::new();
        arg.set_method("GET");
        arg.set_path(url);
        if let Some(param) = param {
            arg.set_param(param);
        }
        arg
    }
    pub fn from_post(url: &str, param: Option<serde_json::Value>) -> Self {
        let mut arg = RequestArg::new();
        arg.set_method("POST");
        arg.set_path(url);
        if let Some(param) = param {
            arg.set_param(param);
        }
        arg
    }
    pub fn from_post_form(url: &str, param: Option<serde_json::Value>) -> Self {
        let mut arg = RequestArg::new();
        arg.set_method("POST");
        arg.set_path(url);
        arg.set_header(("Content-Type", "application/x-www-form-urlencoded"));
        if let Some(param) = param {
            arg.set_param(param);
        }
        arg
    }
    pub fn from_put(url: &str, param: Option<serde_json::Value>) -> Self {
        let mut arg = RequestArg::new();
        arg.set_method("PUT");
        arg.set_path(url);
        if let Some(param) = param {
            arg.set_param(param);
        }
        arg
    }
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
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

pub fn remove_json_string_variable_quota(value: String) -> String {
    let mut inter = value.chars().collect::<Vec<char>>();
    inter.extend(vec!['E', 'D']);
    let mut windows = inter.windows(3);

    let mut variable_start = false;
    let mut result: Vec<char> = vec![];
    let mut last_is_variable_end = false;
    while let Some(sub) = windows.next() {
        let mut need = !last_is_variable_end;
        if sub == ['"', '$', '{'] {
            variable_start = true;
            need = false;
        }

        if variable_start && sub[0] == '}' {
            variable_start = false;
            last_is_variable_end = true;
        } else {
            last_is_variable_end = false;
        }

        if need {
            result.push(sub[0])
        }
    }
    String::from_iter(result)
}
