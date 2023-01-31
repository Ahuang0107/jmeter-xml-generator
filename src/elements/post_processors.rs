use crate::elements::base::string_prop;
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

pub fn json_post_processor(variable_name: &str, path_expr: &str) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("JSONPostProcessor")
            .attr("guiclass", "JSONPostProcessorGui")
            .attr("testclass", "JSONPostProcessor")
            .attr("testname", "JSON Extractor")
            .attr("enabled", "true"),
        vec![
            string_prop("JSONPostProcessor.referenceNames", variable_name),
            string_prop("JSONPostProcessor.jsonPathExprs", path_expr),
            string_prop("JSONPostProcessor.match_numbers", ""),
        ],
    )
    .with_subs(vec![])
}

pub fn groovy_post_processor(script: &str) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("JSR223PostProcessor")
            .attr("guiclass", "TestBeanGUI")
            .attr("testclass", "JSR223PostProcessor")
            .attr("testname", "JSR223 PostProcessor")
            .attr("enabled", "true"),
        vec![
            string_prop("scriptLanguage", "groovy"),
            string_prop("parameters", ""),
            string_prop("filename", ""),
            string_prop("cacheKey", "true"),
            string_prop("script", script),
        ],
    )
    .with_subs(vec![])
}
