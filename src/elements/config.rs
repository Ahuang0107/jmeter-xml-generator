use crate::elements::base::{bool_prop, collection_prop, element_prop, element_props, string_prop};
use crate::generator::CsvDataSetConfig;
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
    .with_subs(vec![])
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
    .with_subs(vec![])
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
    .with_subs(vec![])
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
            string_prop("HTTPSampler.concurrentPool", "6"),
            string_prop("HTTPSampler.connect_timeout", ""),
            string_prop("HTTPSampler.response_timeout", ""),
        ],
    )
    .with_subs(vec![])
}

pub(crate) fn csv_data_set_config(config: &CsvDataSetConfig) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("CSVDataSet")
            .attr("guiclass", "TestBeanGUI")
            .attr("testclass", "CSVDataSet")
            .attr("testname", "CSV Data Set Config")
            .attr("enabled", "true"),
        vec![
            string_prop("filename", config.filename.as_str()),
            string_prop("fileEncoding", ""),
            string_prop("variableNames", ""),
            bool_prop("ignoreFirstLine", false),
            string_prop("delimiter", ","),
            bool_prop("quotedData", false),
            bool_prop("recycle", config.recycle),
            bool_prop("stopThread", config.stop_thread),
            string_prop("shareMode", "shareMode.all"),
        ],
    )
    .with_subs(vec![])
}
