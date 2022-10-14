use crate::elements::{root, Request};
use crate::xml::EventWriter;

#[allow(dead_code)]
pub struct ScriptBuilder {
    host: String,
    port: String,
    num_threads: usize,
    headers: Vec<(String, String)>,
    requests: Vec<Request>,
}

#[allow(dead_code)]
impl ScriptBuilder {
    pub fn new(host: String, port: String, num_threads: usize) -> ScriptBuilder {
        ScriptBuilder {
            host,
            port,
            num_threads,
            headers: Vec::new(),
            requests: Vec::new(),
        }
    }
    pub fn add_header(&mut self, k: String, v: String) {
        self.headers.push((k, v));
    }
    pub fn add_request(&mut self, req: Request) {
        self.requests.push(req);
    }
    pub fn build(&self) -> Vec<u8> {
        let target: Vec<u8> = Vec::new();
        let mut writer = EventWriter::new(target);
        let script = root(
            self.host.clone(),
            self.port.clone(),
            self.headers
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<(String, String)>>(),
            self.requests
                .iter()
                .map(|h| h.clone())
                .collect::<Vec<Request>>(),
        );

        script.write(&mut writer);
        writer.export()
    }
}
