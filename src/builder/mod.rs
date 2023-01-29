use log::info;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::elements::root;
use crate::xml::EventWriter;

#[cfg(test)]
mod test;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ScriptBuilder {
    target: std::rc::Rc<std::cell::RefCell<ScriptGenerator>>,
}

pub struct ScriptGenerator {
    variables: std::collections::HashMap<String, String>,
    headers: std::collections::HashMap<String, String>,
    requests: Vec<RequestArg>,
    timer: instant::Instant,
    switch: bool,
}

#[derive(Clone, Debug)]
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

#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[derive(Debug)]
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
        if split.len() > 1 {
            let path_params_string = split.last().unwrap_or(&"");
            let path_params = path_params_string.split('&').collect::<Vec<&str>>();
            path_params.iter().for_each(|s| {
                let kv = s.split('=').collect::<Vec<&str>>();
                let k = kv.first().unwrap_or(&"");
                let v = kv.last().unwrap_or(&"");
                self.params.insert(k.to_string(), v.to_string());
            });
        }
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
    pub fn new() -> Self {
        let target = std::rc::Rc::new(std::cell::RefCell::new(ScriptGenerator::new()));
        if cfg!(target_arch = "wasm32") {
            let window: web_sys::Window = web_sys::window().expect("no global `window` exists");
            let document: web_sys::Document =
                window.document().expect("should have a document on window");
            let body: web_sys::HtmlElement = document.body().expect("document should have a body");
            let widget: web_sys::HtmlDivElement = document
                .create_element("div")
                .expect("unable to create element")
                .dyn_into::<web_sys::HtmlDivElement>()
                .expect("unable to dyn into HtmlDivElement");
            widget
                .set_attribute("id", "widget")
                .expect("unable to set element attribute");
            widget.style().set_property("position", "fixed").unwrap();
            widget.style().set_property("right", "20px").unwrap();
            widget.style().set_property("top", "40px").unwrap();
            widget.style().set_property("display", "none").unwrap();
            widget
                .style()
                .set_property("flex-direction", "column")
                .unwrap();
            widget.style().set_property("width", "300px").unwrap();
            widget.style().set_property("height", "300px").unwrap();
            widget
                .style()
                .set_property("border", "black 1px solid")
                .unwrap();
            widget.style().set_property("border-radius", "5px").unwrap();
            widget.style().set_property("overflow", "scroll").unwrap();
            let ul: web_sys::HtmlUListElement = document
                .create_element("ul")
                .expect("unable to create element")
                .dyn_into::<web_sys::HtmlUListElement>()
                .expect("unable to dyn into HtmlUListElement");
            ul.set_attribute("id", "widget-ul")
                .expect("unable to set element attribute");
            let button: web_sys::HtmlButtonElement = document
                .create_element("button")
                .unwrap()
                .dyn_into::<web_sys::HtmlButtonElement>()
                .unwrap();
            button.set_text_content(Some("Download"));
            widget.append_child(&button).unwrap();
            widget.append_child(&ul).unwrap();
            body.append_child(&widget)
                .expect("unable to append element");
            {
                let target = target.clone();
                let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {
                    if e.shift_key() && e.key() == "J" {
                        let document: web_sys::Document =
                            window.document().expect("should have a document on window");
                        let mut target = target.borrow_mut();
                        if !target.switch {
                            target.switch = true;
                            document
                                .get_element_by_id("widget")
                                .expect("unable to get element")
                                .dyn_into::<web_sys::HtmlDivElement>()
                                .expect("unable to dyn into HtmlDivElement")
                                .style()
                                .set_property("display", "flex")
                                .unwrap();
                        } else {
                            target.switch = false;
                            document
                                .get_element_by_id("widget")
                                .expect("unable to get element")
                                .dyn_into::<web_sys::HtmlDivElement>()
                                .expect("unable to dyn into HtmlDivElement")
                                .style()
                                .set_property("display", "none")
                                .unwrap();
                        }
                    }
                });
                document
                    .add_event_listener_with_callback("keypress", closure.as_ref().unchecked_ref())
                    .expect("unable to add keypress event listener");
                closure.forget();
            }
            {
                let target = target.clone();
                let closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
                    let result = target.borrow_mut().build();
                    let uint8arr = js_sys::Uint8Array::new(
                        &unsafe { js_sys::Uint8Array::view(&result) }.into(),
                    );
                    let array = js_sys::Array::new();
                    array.push(&uint8arr.buffer());
                    let blob = web_sys::Blob::new_with_u8_array_sequence_and_options(
                        &array,
                        &web_sys::BlobPropertyBag::new().type_("application/xml"),
                    )
                    .unwrap();
                    let download_url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                    let document = web_sys::window().unwrap().document().unwrap();
                    let anchor = document
                        .create_element("a")
                        .expect("unable to create element")
                        .dyn_into::<web_sys::HtmlAnchorElement>()
                        .expect("unable to dyn into HtmlAnchorElement");
                    anchor.style().set_property("display", "none").unwrap();
                    anchor.set_attribute("href", &download_url).unwrap();
                    anchor.set_attribute("download", "Test Plan.jmx").unwrap();
                    document.body().unwrap().append_child(&anchor).unwrap();
                    anchor.click();
                    anchor.remove();
                    web_sys::Url::revoke_object_url(&download_url).unwrap();
                    log::info!("download jmx file")
                });
                button
                    .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
                    .expect("unable to add click event listener");
                closure.forget();
            }
        }
        Self { target }
    }
    pub fn add_header(&mut self, k: String, v: String) {
        self.target.borrow_mut().add_header(k, v)
    }
    /// add axios request
    ///
    /// * `axios_config_raw` - serialise of AxiosConfigRaw
    pub fn add_axios_request(&mut self, axios_config_raw: String) {
        self.target.borrow_mut().add_axios_request(axios_config_raw)
    }
    /// generate the test plan script
    /// return file data
    pub fn build(&self) -> Vec<u8> {
        self.target.borrow_mut().build()
    }
    #[cfg(test)]
    pub fn mock() -> Self {
        let target = std::rc::Rc::new(std::cell::RefCell::new(ScriptGenerator::mock()));
        Self { target }
    }
}

impl ScriptGenerator {
    pub fn new() -> ScriptGenerator {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        wasm_logger::init(wasm_logger::Config::default());
        ScriptGenerator {
            variables: std::collections::HashMap::new(),
            headers: std::collections::HashMap::new(),
            requests: Vec::new(),
            timer: instant::Instant::now(),
            switch: false,
        }
    }
    pub fn add_header(&mut self, k: String, v: String) {
        if !self.switch {
            return;
        }
        self.headers.insert(k, v);
    }
    pub fn add_axios_request(&mut self, axios_config_raw: String) {
        if !self.switch {
            return;
        }
        let last_time = self.timer;
        self.timer = instant::Instant::now();
        let during = self.timer.duration_since(last_time).as_millis();
        let request_arg = AxiosConfigRaw::from_str(axios_config_raw.as_str());
        let request_arg = request_arg.into_axios_config();
        let request_arg = request_arg.into_request_arg(during);
        self.requests.push(request_arg);
        if cfg!(target_arch = "wasm32") {
            let show = self.requests.last().unwrap().path.clone();
            let document: web_sys::Document = web_sys::window().unwrap().document().unwrap();
            let li: web_sys::HtmlLiElement = document
                .create_element("li")
                .expect("unable to create element")
                .dyn_into::<web_sys::HtmlLiElement>()
                .expect("unable to dyn into HtmlLiElement");
            li.set_text_content(Some(&show));
            document
                .get_element_by_id("widget-ul")
                .expect("unable to get element")
                .dyn_into::<web_sys::HtmlUListElement>()
                .expect("unable to dyn into HtmlUListElement")
                .append_child(&li)
                .unwrap();
            info!("succeed add request")
        }
    }
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
    #[cfg(test)]
    pub fn mock() -> ScriptGenerator {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        wasm_logger::init(wasm_logger::Config::default());
        ScriptGenerator {
            variables: std::collections::HashMap::new(),
            headers: std::collections::HashMap::new(),
            requests: Vec::new(),
            timer: instant::Instant::now(),
            switch: true,
        }
    }
}
