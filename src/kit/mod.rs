use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::axios_config::AxiosRequestConfig;
use crate::generator::ScriptGenerator;

#[wasm_bindgen]
#[derive(Clone)]
pub struct ScriptGeneratorKit {
    target: std::rc::Rc<std::cell::RefCell<ScriptGenerator>>,
    switch: std::rc::Rc<std::cell::RefCell<bool>>,
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
impl ScriptGeneratorKit {
    /// create a script kit that can generate a jmeter test plan script
    pub fn new() -> Self {
        let target = std::rc::Rc::new(std::cell::RefCell::new(ScriptGenerator::new()));
        let switch = std::rc::Rc::new(std::cell::RefCell::new(false));

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
            let switch = switch.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::KeyboardEvent| {
                if e.shift_key() && e.key() == "J" {
                    let document: web_sys::Document =
                        window.document().expect("should have a document on window");
                    let mut switch = switch.borrow_mut();
                    if !*switch {
                        *switch = true;
                        document
                            .get_element_by_id("widget")
                            .expect("unable to get element")
                            .dyn_into::<web_sys::HtmlDivElement>()
                            .expect("unable to dyn into HtmlDivElement")
                            .style()
                            .set_property("display", "flex")
                            .unwrap();
                    } else {
                        *switch = false;
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
                let uint8arr =
                    js_sys::Uint8Array::new(&unsafe { js_sys::Uint8Array::view(&result) }.into());
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
        Self { target, switch }
    }
    /// add axios request
    ///
    /// * `axios_request_config` - serialise of AxiosRequestConfig
    pub fn add_axios_request(&mut self, axios_request_config: JsValue) {
        // self.target.borrow_mut().add_axios_request(axios_config_raw)
        let axios_request_config = AxiosRequestConfig::from(axios_request_config);
        todo!()
    }
    /// generate the test plan script
    /// return file data
    pub fn build(&self) -> Vec<u8> {
        self.target.borrow_mut().build()
    }
}
