use wasm_bindgen::prelude::*;

use crate::elements::root;
use crate::xml::EventWriter;

#[allow(dead_code)]
#[wasm_bindgen]
pub struct ScriptBuilder {
    host: String,
    port: String,
    num_threads: usize,
    headers: Vec<(String, String)>,
    requests: Vec<(Request, String)>,
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Request {
    GET(String),
    POST {
        payload: String,
        with_form_data: bool,
    },
    PUT(String),
}

#[allow(dead_code)]
#[wasm_bindgen]
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
    fn add_request(&mut self, req: Request, url: String) {
        self.requests.push((req, url));
    }
    pub fn get(&mut self, url: String, payload: String) {
        self.add_request(Request::GET(payload), url)
    }
    pub fn post(&mut self, url: String, payload: String) {
        self.add_request(
            Request::POST {
                payload,
                with_form_data: false,
            },
            url,
        )
    }
    pub fn post_with_form_data(&mut self, url: String, payload: String) {
        self.add_request(
            Request::POST {
                payload,
                with_form_data: true,
            },
            url,
        )
    }
    pub fn put(&mut self, url: String, payload: String) {
        self.add_request(Request::PUT(payload), url)
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
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<(Request, String)>>(),
        );

        script.write(&mut writer);
        writer.export()
    }
}
