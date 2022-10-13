use xml::writer::XmlEvent;

use crate::elements::base::{bool_prop, collection_prop, element_prop, string_prop};
use crate::script::ScriptElement;

#[allow(dead_code)]
pub(crate) fn get_simple(path: &'static str) -> ScriptElement<'static> {
    http_sampler_proxy(path, "GET", false, vec![])
}

#[allow(dead_code)]
pub(crate) fn post_simple(path: &'static str) -> ScriptElement<'static> {
    http_sampler_proxy(path, "GET", false, vec![])
}

#[allow(dead_code)]
pub(crate) fn post_multipart(
    path: &'static str,
    args: Vec<(&'static str, &'static str)>,
) -> ScriptElement<'static> {
    http_sampler_proxy(path, "POST", true, args)
}

#[allow(dead_code)]
fn http_sampler_proxy(
    path: &'static str,
    method: &'static str,
    multipart: bool,
    args: Vec<(&'static str, &'static str)>,
) -> ScriptElement<'static> {
    ScriptElement::from(
        XmlEvent::start_element("HTTPSamplerProxy")
            .attr("guiclass", "HttpTestSampleGui")
            .attr("testclass", "HTTPSamplerProxy")
            .attr("testname", path)
            .attr("enabled", "true"),
        vec![
            arguments(
                args.into_iter()
                    .map(|a| argument(a.0, a.1))
                    .collect::<Vec<ScriptElement>>(),
            ),
            string_prop("HTTPSampler.domain", ""),
            string_prop("HTTPSampler.port", ""),
            string_prop("HTTPSampler.protocol", ""),
            string_prop("HTTPSampler.contentEncoding", ""),
            string_prop("HTTPSampler.path", path),
            string_prop("HTTPSampler.method", method),
            bool_prop("HTTPSampler.follow_redirects", true),
            bool_prop("HTTPSampler.auto_redirects", false),
            bool_prop("HTTPSampler.use_keepalive", true),
            bool_prop("HTTPSampler.DO_MULTIPART_POST", multipart),
            string_prop("HTTPSampler.embedded_url_re", ""),
            string_prop("HTTPSampler.connect_timeout", ""),
            string_prop("HTTPSampler.response_timeout", ""),
        ],
    )
}

#[allow(dead_code)]
fn arguments(args: Vec<ScriptElement<'static>>) -> ScriptElement<'static> {
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
fn argument(name: &'static str, value: &'static str) -> ScriptElement<'static> {
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
