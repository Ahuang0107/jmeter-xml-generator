use crate::elements::base::string_prop;
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

pub fn groovy_pre_processor(script: &str) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("JSR223PreProcessor")
            .attr("guiclass", "TestBeanGUI")
            .attr("testclass", "JSR223PreProcessor")
            .attr("testname", "JSR223 PreProcessor")
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
