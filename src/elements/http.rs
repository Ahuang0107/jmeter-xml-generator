use xml::writer::XmlEvent;

use crate::elements::base::{bool_prop, collection_prop, element_prop, string_prop};
use crate::script::ScriptElement;

#[allow(dead_code)]
pub struct Request<'a> {
    path: &'a str,
    method: &'a str,
    multipart: bool,
    args: Vec<(&'a str, &'a str)>,
    body: Option<&'a str>,
}

impl<'a> Clone for Request<'a> {
    fn clone(&self) -> Self {
        let clone = Request {
            path: self.path.clone(),
            method: self.method.clone(),
            multipart: self.multipart.clone(),
            args: self
                .args
                .iter()
                .map(|h| (h.0, h.1))
                .collect::<Vec<(&'a str, &'a str)>>(),
            body: self.body.clone(),
        };
        clone
    }
}

#[allow(dead_code)]
impl<'a> Request<'a> {
    pub fn from(
        path: &'a str,
        method: &'a str,
        multipart: bool,
        args: Vec<(&'a str, &'a str)>,
        body: Option<&'a str>,
    ) -> Request<'a> {
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
        XmlEvent::start_element("HTTPSamplerProxy")
            .attr("guiclass", "HttpTestSampleGui")
            .attr("testclass", "HTTPSamplerProxy")
            .attr("testname", req.path)
            .attr("enabled", "true"),
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
            string_prop("HTTPSampler.domain", ""),
            string_prop("HTTPSampler.port", ""),
            string_prop("HTTPSampler.protocol", ""),
            string_prop("HTTPSampler.contentEncoding", ""),
            string_prop("HTTPSampler.path", req.path),
            string_prop("HTTPSampler.method", req.method),
            bool_prop("HTTPSampler.follow_redirects", true),
            bool_prop("HTTPSampler.auto_redirects", false),
            bool_prop("HTTPSampler.use_keepalive", true),
            bool_prop("HTTPSampler.DO_MULTIPART_POST", req.multipart),
            string_prop("HTTPSampler.embedded_url_re", ""),
            string_prop("HTTPSampler.connect_timeout", ""),
            string_prop("HTTPSampler.response_timeout", ""),
        ],
    )
}

#[allow(dead_code)]
fn arguments(args: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("elementProp")
            .attr("name", "HTTPsampler.Arguments")
            .attr("elementType", "Arguments")
            .attr("guiclass", "HTTPArgumentsPanel")
            .attr("testclass", "Arguments")
            .attr("testname", "User Defined Variables")
            .attr("enabled", "true"),
        vec![collection_prop("Arguments.arguments", args)],
    )
}

#[allow(dead_code)]
fn argument<'a>(name: &'a str, value: &'a str) -> ScriptElement<'a> {
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
