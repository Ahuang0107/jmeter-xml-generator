use crate::elements::base::{bool_prop, collection_prop, element_prop, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub struct Request {
    path: String,
    method: String,
    multipart: bool,
    args: Vec<(String, String)>,
    body: Option<String>,
}

impl Clone for Request {
    fn clone(&self) -> Self {
        let clone = Request {
            path: self.path.clone(),
            method: self.method.clone(),
            multipart: self.multipart.clone(),
            args: self
                .args
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect::<Vec<(String, String)>>(),
            body: self.body.clone(),
        };
        clone
    }
}

#[allow(dead_code)]
impl Request {
    pub fn from(
        path: String,
        method: String,
        multipart: bool,
        args: Vec<(String, String)>,
        body: Option<String>,
    ) -> Request {
        Request {
            path,
            method,
            multipart,
            args,
            body,
        }
    }
    pub fn with_json(&self) -> bool {
        self.method == "post" && self.body.is_some()
    }
}

#[allow(dead_code)]
pub fn http_sampler_proxy(req: Request) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("HTTPSamplerProxy".to_string())
            .attr("guiclass".to_string(), "HttpTestSampleGui".to_string())
            .attr("testclass".to_string(), "HTTPSamplerProxy".to_string())
            .attr("testname".to_string(), req.path.clone())
            .attr("enabled".to_string(), "true".to_string()),
        vec![
            match req.body {
                Some(data) => arguments(vec![body_json(data)]),
                None => arguments(
                    req.args
                        .into_iter()
                        .map(|a| argument(a.0, a.1))
                        .collect::<Vec<ScriptElement>>(),
                ),
            },
            string_prop("HTTPSampler.domain".to_string(), "".to_string()),
            string_prop("HTTPSampler.port".to_string(), "".to_string()),
            string_prop("HTTPSampler.protocol".to_string(), "".to_string()),
            string_prop("HTTPSampler.contentEncoding".to_string(), "".to_string()),
            string_prop("HTTPSampler.path".to_string(), req.path),
            string_prop("HTTPSampler.method".to_string(), req.method),
            bool_prop("HTTPSampler.follow_redirects".to_string(), true),
            bool_prop("HTTPSampler.auto_redirects".to_string(), false),
            bool_prop("HTTPSampler.use_keepalive".to_string(), true),
            bool_prop("HTTPSampler.DO_MULTIPART_POST".to_string(), req.multipart),
            string_prop("HTTPSampler.embedded_url_re".to_string(), "".to_string()),
            string_prop("HTTPSampler.connect_timeout".to_string(), "".to_string()),
            string_prop("HTTPSampler.response_timeout".to_string(), "".to_string()),
        ],
    )
}

#[allow(dead_code)]
fn arguments(args: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("elementProp".to_string())
            .attr("name".to_string(), "HTTPsampler.Arguments".to_string())
            .attr("elementType".to_string(), "Arguments".to_string())
            .attr("guiclass".to_string(), "HTTPArgumentsPanel".to_string())
            .attr("testclass".to_string(), "Arguments".to_string())
            .attr("testname".to_string(), "User Defined Variables".to_string())
            .attr("enabled".to_string(), "true".to_string()),
        vec![collection_prop("Arguments.arguments".to_string(), args)],
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
