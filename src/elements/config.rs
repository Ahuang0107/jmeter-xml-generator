use crate::elements::base::{bool_prop, collection_prop, element_prop, element_props, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

pub fn arguments(variables: &Vec<(String, String)>) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("Arguments")
            .attr("guiclass", "ArgumentsPanel")
            .attr("testclass", "Arguments")
            .attr("testname", "User Defined Variables")
            .attr("enabled", "true"),
        vec![collection_prop(
            "Arguments.arguments",
            variables
                .into_iter()
                .map(|(k, v)| {
                    element_prop(
                        k,
                        "Argument",
                        vec![
                            string_prop("Argument.name", k),
                            string_prop("Argument.value", v),
                            string_prop("Argument.metadata", "="),
                        ],
                    )
                })
                .collect(),
        )],
    )
    .add_subs(vec![])
}

pub(crate) fn header_manager(headers: &Vec<(String, String)>) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("HeaderManager")
            .attr("guiclass", "HeaderPanel")
            .attr("testclass", "HeaderManager")
            .attr("testname", "HTTP Header Manager")
            .attr("enabled", "true"),
        vec![collection_prop(
            "HeaderManager.headers",
            headers
                .into_iter()
                .map(|(k, v)| header(k, v))
                .collect::<Vec<ScriptElement>>(),
        )],
    )
    .add_subs(vec![])
}

pub(crate) fn cookie_manager() -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("CookieManager")
            .attr("guiclass", "CookiePanel")
            .attr("testclass", "CookieManager")
            .attr("testname", "HTTP Cookie Manager")
            .attr("enabled", "true"),
        vec![
            ScriptElement::from_str(
                XmlEvent::start_element("collectionProp").attr("name", "CookieManager.cookies"),
                "",
            ),
            bool_prop("CookieManager.clearEachIteration", false),
            bool_prop("CookieManager.controlledByThreadGroup", false),
        ],
    )
    .add_subs(vec![])
}

pub(crate) fn header(name: &str, value: &str) -> ScriptElement {
    element_prop(
        "",
        "Header",
        vec![
            string_prop("Header.name", name),
            string_prop("Header.value", value),
        ],
    )
}

pub(crate) fn constant_timer(delay: u128) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("ConstantTimer")
            .attr("guiclass", "ConstantTimerGui")
            .attr("testclass", "ConstantTimer")
            .attr("testname", "Constant Timer")
            .attr("enabled", "true"),
        vec![string_prop(
            "ConstantTimer.delay",
            delay.to_string().as_str(),
        )],
    )
    .add_subs(vec![])
}

pub(crate) fn request_default(protocol: &str, host: &str, port: &str) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("ConfigTestElement")
            .attr("guiclass", "HttpDefaultsGui")
            .attr("testclass", "ConfigTestElement")
            .attr("testname", "HTTP Request Defaults")
            .attr("enabled", "true"),
        vec![
            element_props("HTTPsampler.Arguments", vec![]),
            string_prop("HTTPSampler.domain", host),
            string_prop("HTTPSampler.port", port),
            string_prop("HTTPSampler.protocol", protocol),
            string_prop("HTTPSampler.contentEncoding", ""),
            string_prop("HTTPSampler.path", ""),
            string_prop("HTTPSampler.concurrentPool", ""),
            string_prop("HTTPSampler.connect_timeout", ""),
            string_prop("HTTPSampler.response_timeout", ""),
        ],
    )
    .add_subs(vec![])
}
