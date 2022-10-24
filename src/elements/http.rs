use crate::builder::Request;
use crate::elements::base::{arguments, bool_prop, element_prop, string_prop};
use crate::script::ScriptElement;
use crate::utils::KeyValue;
use crate::xml::XmlEvent;

impl Request {
    pub fn method(&self) -> &str {
        match self {
            Request::GET(_) => "GET",
            Request::POST {
                payload: _payload,
                with_form_data: _with_form_data,
            } => "POST",
            Request::PUT(_) => "PUT",
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
    pub fn is_put(&self) -> bool {
        match self {
            Request::GET(_) => false,
            Request::POST {
                payload: _payload,
                with_form_data: _with_form_data,
            } => false,
            Request::PUT(_) => true,
        }
    }
}

#[allow(dead_code)]
pub fn http_sampler_proxy(request: Request, request_url: &str) -> ScriptElement {
    let method = request.method();
    let with_form_data = request.with_form_data();
    ScriptElement::from(
        XmlEvent::start_element("HTTPSamplerProxy")
            .attr("guiclass", "HttpTestSampleGui")
            .attr("testclass", "HTTPSamplerProxy")
            .attr("testname", request_url.clone())
            .attr("enabled", "true"),
        vec![
            match request.clone() {
                Request::GET(payload) => arguments(
                    "HTTPsampler.Arguments",
                    serde_json::from_str::<Vec<KeyValue>>(payload.as_str())
                        .unwrap_or(vec![])
                        .into_iter()
                        .map(|kv| argument(kv.key.as_str(), kv.value.as_str()))
                        .collect::<Vec<ScriptElement>>(),
                ),
                Request::POST {
                    payload,
                    with_form_data,
                } => {
                    if !with_form_data {
                        arguments("HTTPsampler.Arguments", vec![body_json(payload.as_str())])
                    } else {
                        arguments(
                            "HTTPsampler.Arguments",
                            serde_json::from_str::<Vec<KeyValue>>(payload.as_str())
                                .unwrap()
                                .into_iter()
                                .map(|kv| argument(kv.key.as_str(), kv.value.as_str()))
                                .collect(),
                        )
                    }
                }
                Request::PUT(payload) => {
                    arguments("HTTPsampler.Arguments", vec![body_json(payload.as_str())])
                }
            },
            string_prop("HTTPSampler.domain", ""),
            string_prop("HTTPSampler.port", ""),
            string_prop("HTTPSampler.protocol", ""),
            string_prop("HTTPSampler.contentEncoding", ""),
            string_prop("HTTPSampler.path", request_url.clone()),
            string_prop("HTTPSampler.method", method),
            bool_prop("HTTPSampler.follow_redirects", true),
            bool_prop("HTTPSampler.auto_redirects", false),
            bool_prop("HTTPSampler.use_keepalive", true),
            bool_prop("HTTPSampler.DO_MULTIPART_POST", with_form_data),
            string_prop("HTTPSampler.embedded_url_re", ""),
            string_prop("HTTPSampler.connect_timeout", ""),
            string_prop("HTTPSampler.response_timeout", ""),
        ],
    )
}

#[allow(dead_code)]
fn argument(name: &str, value: &str) -> ScriptElement {
    element_prop(
        "",
        "HTTPArgument",
        vec![
            bool_prop("HTTPArgument.always_encode", true),
            string_prop("Argument.value", value),
            string_prop("Argument.metadata", "="),
            bool_prop("HTTPArgument.use_equals", true),
            string_prop("Argument.name", name),
        ],
    )
}

#[allow(dead_code)]
fn body_json(value: &str) -> ScriptElement {
    element_prop(
        "",
        "HTTPArgument",
        vec![
            bool_prop("HTTPArgument.always_encode", false),
            string_prop("Argument.value", value),
            string_prop("Argument.metadata", "="),
        ],
    )
}
