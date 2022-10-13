use xml::EmitterConfig;

use crate::elements::root;
use crate::script::ScriptElement;

#[allow(dead_code)]
pub struct ScriptBuilder<'a> {
    host: &'a str,
    port: &'a str,
    num_threads: usize,
    headers: Vec<(&'a str, &'a str)>,
    requests: Vec<ScriptElement<'a>>,
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
        );

        script.write(&mut writer);
        target
    }
}
