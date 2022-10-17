use crate::builder::Request;
use crate::elements::base::{arguments, bool_prop, element_prop, string_prop};
use crate::script::ScriptElement;
use crate::utils::KeyValue;
use crate::xml::XmlEvent;

impl Request {
    pub fn method(&self) -> String {
        match self {
            Request::GET(_) => "GET".to_string(),
            Request::POST {
                payload: _payload,
                with_form_data: _with_form_data,
            } => "POST".to_string(),
            Request::PUT(_) => "PUT".to_string(),
        }
    }
    pub fn with_form_data(&self) -> bool {
        match self {
            Request::GET(_) => false,
            Request::POST {
                payload: _payload,
                with_form_data,
            } => with_form_data.clone(),
            Request::PUT(_) => false,
        }
    }
    pub fn is_post(&self) -> bool {
        match self {
            Request::GET(_) => false,
            Request::POST {
                payload: _payload,
                with_form_data: _with_form_data,
            } => true,
            Request::PUT(_) => false,
        }
    }
}

#[allow(dead_code)]
pub fn http_sampler_proxy(request: Request, request_url: String) -> ScriptElement {
    let method = request.method();
    let with_form_data = request.with_form_data();
    ScriptElement::from(
        XmlEvent::start_element("HTTPSamplerProxy".to_string())
            .attr("guiclass".to_string(), "HttpTestSampleGui".to_string())
            .attr("testclass".to_string(), "HTTPSamplerProxy".to_string())
            .attr("testname".to_string(), request_url.clone())
            .attr("enabled".to_string(), "true".to_string()),
        vec![
            match request {
                Request::GET(payload) => arguments(
                    "HTTPsampler.Arguments".to_string(),
                    vec![body_json(payload)],
                ),
                Request::POST {
                    payload,
                    with_form_data,
                } => {
                    if !with_form_data {
                        arguments(
                            "HTTPsampler.Arguments".to_string(),
                            vec![body_json(payload)],
                        )
                    } else {
                        arguments(
                            "HTTPsampler.Arguments".to_string(),
                            serde_json::from_str::<Vec<KeyValue>>(payload.as_str())
                                .unwrap()
                                .into_iter()
                                .map(|kv| argument(kv.key, kv.value))
                                .collect(),
                        )
                    }
                }
                Request::PUT(payload) => arguments(
                    "HTTPsampler.Arguments".to_string(),
                    vec![body_json(payload)],
                ),
            },
            string_prop("HTTPSampler.domain".to_string(), "".to_string()),
            string_prop("HTTPSampler.port".to_string(), "".to_string()),
            string_prop("HTTPSampler.protocol".to_string(), "".to_string()),
            string_prop("HTTPSampler.contentEncoding".to_string(), "".to_string()),
            string_prop("HTTPSampler.path".to_string(), request_url.clone()),
            string_prop("HTTPSampler.method".to_string(), method),
            bool_prop("HTTPSampler.follow_redirects".to_string(), true),
            bool_prop("HTTPSampler.auto_redirects".to_string(), false),
            bool_prop("HTTPSampler.use_keepalive".to_string(), true),
            bool_prop("HTTPSampler.DO_MULTIPART_POST".to_string(), with_form_data),
            string_prop("HTTPSampler.embedded_url_re".to_string(), "".to_string()),
            string_prop("HTTPSampler.connect_timeout".to_string(), "".to_string()),
            string_prop("HTTPSampler.response_timeout".to_string(), "".to_string()),
        ],
    )
}

#[allow(dead_code)]
fn argument(name: String, value: String) -> ScriptElement {
    element_prop(
        "".to_string(),
        "HTTPArgument".to_string(),
        vec![
            bool_prop("HTTPArgument.always_encode".to_string(), true),
            string_prop("Argument.value".to_string(), value),
            string_prop("Argument.metadata".to_string(), "=".to_string()),
            bool_prop("HTTPArgument.use_equals".to_string(), true),
            string_prop("Argument.name".to_string(), name),
        ],
    )
}

#[allow(dead_code)]
fn body_json(value: String) -> ScriptElement {
    element_prop(
        "".to_string(),
        "HTTPArgument".to_string(),
        vec![
            bool_prop("HTTPArgument.always_encode".to_string(), false),
            string_prop("Argument.value".to_string(), value),
            string_prop("Argument.metadata".to_string(), "=".to_string()),
        ],
    )
}
