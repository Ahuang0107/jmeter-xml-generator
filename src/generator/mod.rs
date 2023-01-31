use crate::elements::root;
use crate::request::RequestArg;
use crate::xml::EventWriter;

pub struct ScriptGenerator {
    // User Defined Variables
    variables: Vec<(String, String)>,
    // HTTP Header Manager
    headers: Vec<(String, String)>,
    // HTTP Request Default
    protocol: String,
    host: String,
    port: String,
    once_requests: Vec<RequestArg>,
    requests: Vec<RequestArg>,
    timer: instant::Instant,
}

impl ScriptGenerator {
    pub fn new() -> ScriptGenerator {
        ScriptGenerator {
            variables: vec![],
            headers: vec![],
            protocol: String::new(),
            host: String::new(),
            port: String::new(),
            once_requests: Vec::new(),
            requests: Vec::new(),
            timer: instant::Instant::now(),
        }
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
    pub fn add_header(&mut self, k: &str, v: &str) {
        self.headers.push((k.to_string(), v.to_string()));
    }
    pub fn once_get(&mut self, url: &str, param: Option<serde_json::Value>) {
        let mut arg = RequestArg::new();
        arg.set_method("GET");
        arg.set_path(url);
        if let Some(param) = param {
            arg.set_param(param);
        }
        self.once_requests.push(arg);
    }
    pub fn get(&mut self, url: &str, param: Option<serde_json::Value>) {
        let mut arg = RequestArg::new();
        arg.set_method("GET");
        arg.set_path(url);
        if let Some(param) = param {
            arg.set_param(param);
        }
        self.requests.push(arg);
    }
    pub fn once_post(&mut self, url: &str, param: Option<serde_json::Value>) {
        let mut arg = RequestArg::new();
        arg.set_method("POST");
        arg.set_path(url);
        if let Some(param) = param {
            arg.set_param(param);
        }
        self.once_requests.push(arg);
    }
    pub fn post(&mut self, url: &str, param: Option<serde_json::Value>) {
        let mut arg = RequestArg::new();
        arg.set_method("POST");
        arg.set_path(url);
        if let Some(param) = param {
            arg.set_param(param);
        }
        self.requests.push(arg);
    }
    pub fn once_post_form_data(&mut self, url: &str, param: Option<serde_json::Value>) {
        let mut arg = RequestArg::new();
        arg.set_method("POST");
        arg.set_path(url);
        arg.set_header(("Content-Type", "multipart/form-data"));
        if let Some(param) = param {
            arg.set_param(param);
        }
        self.once_requests.push(arg);
    }
    pub fn post_form_data(&mut self, url: &str, param: Option<serde_json::Value>) {
        let mut arg = RequestArg::new();
        arg.set_method("POST");
        arg.set_path(url);
        arg.set_header(("Content-Type", "multipart/form-data"));
        if let Some(param) = param {
            arg.set_param(param);
        }
        self.requests.push(arg);
    }
    pub fn once_put(&mut self, url: &str, param: Option<serde_json::Value>) {
        let mut arg = RequestArg::new();
        arg.set_method("PUT");
        arg.set_path(url);
        if let Some(param) = param {
            arg.set_param(param);
        }
        self.once_requests.push(arg);
    }
    pub fn put(&mut self, url: &str, param: Option<serde_json::Value>) {
        let mut arg = RequestArg::new();
        arg.set_method("PUT");
        arg.set_path(url);
        if let Some(param) = param {
            arg.set_param(param);
        }
        self.requests.push(arg);
    }
    pub fn build(&self) -> Vec<u8> {
        let target: Vec<u8> = Vec::new();
        let mut writer = EventWriter::new(target);
        let script = root(
            &self.protocol,
            &self.host,
            &self.port,
            &self.variables,
            &self.headers,
            &self.once_requests,
            &self.requests,
        );

        script.write(&mut writer);
        writer.export()
    }
}

#[cfg(test)]
mod test;
