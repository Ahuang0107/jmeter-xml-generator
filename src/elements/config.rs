use crate::elements::base::{bool_prop, collection_prop, element_prop, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub(crate) fn config_test_element(host: String, port: String) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("ConfigTestElement".to_string())
            .attr("guiclass".to_string(), "HttpDefaultsGui".to_string())
            .attr("testclass".to_string(), "ConfigTestElement".to_string())
            .attr("testname".to_string(), "HTTP Request Defaults".to_string())
            .attr("enabled".to_string(), "true".to_string()),
        vec![
            arguments(vec![]),
            string_prop("HTTPSampler.domain".to_string(), host),
            string_prop("HTTPSampler.port".to_string(), port),
            string_prop("HTTPSampler.protocol".to_string(), "".to_string()),
            string_prop("HTTPSampler.response_timeout".to_string(), "".to_string()),
        ],
    )
}

#[allow(dead_code)]
pub(crate) fn header_manager(headers: Vec<(String, String)>) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("HeaderManager".to_string())
            .attr("guiclass".to_string(), "HeaderPanel".to_string())
            .attr("testclass".to_string(), "HeaderManager".to_string())
            .attr("testname".to_string(), "HTTP Header Manager".to_string())
            .attr("enabled".to_string(), "true".to_string()),
        vec![collection_prop(
            "HeaderManager.headers".to_string(),
            headers
                .into_iter()
                .map(|a| header(a.0, a.1))
                .collect::<Vec<ScriptElement>>(),
        )],
    )
}

#[allow(dead_code)]
pub(crate) fn cookie_manager() -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("CookieManager".to_string())
            .attr("guiclass".to_string(), "CookiePanel".to_string())
            .attr("testclass".to_string(), "CookieManager".to_string())
            .attr("testname".to_string(), "HTTP Cookie Manager".to_string())
            .attr("enabled".to_string(), "true".to_string()),
        vec![
            ScriptElement::from_empty(
                XmlEvent::start_element("collectionProp".to_string())
                    .attr("name".to_string(), "CookieManager.cookies".to_string()),
            ),
            bool_prop("CookieManager.clearEachIteration".to_string(), false),
            bool_prop("CookieManager.controlledByThreadGroup".to_string(), false),
        ],
    )
}

#[allow(dead_code)]
pub(crate) fn header(name: String, value: String) -> ScriptElement {
    element_prop(
        "".to_string(),
        "Header".to_string(),
        vec![
            string_prop("Header.name".to_string(), name),
            string_prop("Header.value".to_string(), value),
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
