use xml::writer::XmlEvent;

use crate::elements::base::{bool_prop, collection_prop, element_prop, string_prop};
use crate::script::ScriptElement;

#[allow(dead_code)]
pub(crate) fn config_test_element<'a>(
    host: &'a str,
    port: &'a str,
) -> ScriptElement<'a> {
    ScriptElement::from(
        XmlEvent::start_element("ConfigTestElement")
            .attr("guiclass", "HttpDefaultsGui")
            .attr("testclass", "ConfigTestElement")
            .attr("testname", "HTTP Request Defaults")
            .attr("enabled", "true"),
        vec![
            arguments(vec![]),
            string_prop("HTTPSampler.domain", host),
            string_prop("HTTPSampler.port", port),
            string_prop("HTTPSampler.protocol", ""),
            string_prop("HTTPSampler.response_timeout", ""),
        ],
    )
}

#[allow(dead_code)]
pub(crate) fn header_manager<'a>(headers: Vec<(&'a str, &'a str)>) -> ScriptElement<'a> {
    ScriptElement::from(
        XmlEvent::start_element("HeaderManager")
            .attr("guiclass", "HeaderPanel")
            .attr("testclass", "HeaderManager")
            .attr("testname", "HTTP Header Manager")
            .attr("enabled", "true"),
        vec![collection_prop(
            "HeaderManager.headers",
            headers
                .into_iter()
                .map(|a| header(a.0, a.1))
                .collect::<Vec<ScriptElement>>(),
        )],
    )
}

#[allow(dead_code)]
pub(crate) fn cookie_manager<'a>() -> ScriptElement<'a> {
    ScriptElement::from(
        XmlEvent::start_element("CookieManager")
            .attr("guiclass", "CookiePanel")
            .attr("testclass", "CookieManager")
            .attr("testname", "HTTP Cookie Manager")
            .attr("enabled", "true"),
        vec![
            ScriptElement::from_empty(
                XmlEvent::start_element("collectionProp").attr("name", "CookieManager.cookies"),
            ),
            bool_prop("CookieManager.clearEachIteration", false),
            bool_prop("CookieManager.controlledByThreadGroup", false),
        ],
    )
}

#[allow(dead_code)]
pub(crate) fn header<'a>(name: &'a str, value: &'a str) -> ScriptElement<'a> {
    element_prop(
        "",
        "Header",
        vec![
            string_prop("Header.name", name),
            string_prop("Header.value", value),
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
