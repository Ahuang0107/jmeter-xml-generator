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
    requests: Vec<(Request, String, usize)>,
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
    /// create a script builder that can generate a jmeter test plan script
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
    fn add_request(&mut self, req: Request, url: String, delay_time: usize) {
        self.requests.push((req, url, delay_time));
    }
    /// add a get request with a delay time before send this request
    pub fn get(&mut self, url: String, payload: String, delay_time: usize) {
        self.add_request(Request::GET(payload), url, delay_time)
    }
    /// add a post request with a delay time before send this request
    /// send request with json data
    pub fn post(&mut self, url: String, payload: String, delay_time: usize) {
        self.add_request(
            Request::POST {
                payload,
                with_form_data: false,
            },
            url,
            delay_time,
        )
    }
    /// add a post request with a delay time before send this request
    /// send request with form data
    pub fn post_with_form_data(&mut self, url: String, payload: String, delay_time: usize) {
        self.add_request(
            Request::POST {
                payload,
                with_form_data: true,
            },
            url,
            delay_time,
        )
    }
    /// add a put request with a delay time before send this request
    /// send request with json data
    pub fn put(&mut self, url: String, payload: String, delay_time: usize) {
        self.add_request(Request::PUT(payload), url, delay_time)
    }
    /// generate the test plan script
    /// return file data
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
                .map(|(k, v, t)| (k.clone(), v.clone(), t.clone()))
                .collect::<Vec<(Request, String, usize)>>(),
        );

        script.write(&mut writer);
        writer.export()
    }
}
