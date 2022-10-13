use xml::EmitterConfig;

use crate::elements::{root, Request};

#[allow(dead_code)]
pub struct ScriptBuilder<'a> {
    host: &'a str,
    port: &'a str,
    num_threads: usize,
    headers: Vec<(&'a str, &'a str)>,
    requests: Vec<Request<'a>>,
}

#[allow(dead_code)]
impl<'a> ScriptBuilder<'a> {
    pub fn new(host: &'a str, port: &'a str, num_threads: usize) -> ScriptBuilder<'a> {
        ScriptBuilder {
            host,
            port,
            num_threads,
            headers: Vec::new(),
            requests: Vec::new(),
        }
    }
    pub fn add_header(&mut self, k: &'a str, v: &'a str) {
        self.headers.push((k, v));
    }
    pub fn add_request(&mut self, req: Request<'a>) {
        self.requests.push(req);
    }
    pub fn build(&self) -> Vec<u8> {
        let mut target: Vec<u8> = Vec::new();

        let mut writer = EmitterConfig::new().create_writer(&mut target);
        let script = root(
            self.host,
            self.port,
            self.headers
                .iter()
                .map(|h| (h.0, h.1))
                .collect::<Vec<(&'a str, &'a str)>>(),
            self.requests
                .iter()
                .map(|h| h.clone())
                .collect::<Vec<Request<'a>>>(),
        );

        script.write(&mut writer);
        target
    }
}
