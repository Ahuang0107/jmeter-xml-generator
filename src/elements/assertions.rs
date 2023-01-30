use crate::elements::base::{bool_prop, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub fn json_path_assertion(json_path: &str, expected_value: &str) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("JSONPathAssertion")
            .attr("guiclass", "JSONPathAssertionGui")
            .attr("testclass", "JSONPathAssertion")
            .attr("testname", "JSON Assertion")
            .attr("enabled", "true"),
        vec![
            string_prop("JSON_PATH", json_path),
            string_prop("EXPECTED_VALUE", expected_value),
            bool_prop("JSONVALIDATION", true),
            bool_prop("EXPECT_NULL", false),
            bool_prop("INVERT", false),
            bool_prop("ISREGEX", true),
        ],
    )
    .add_subs(vec![])
}
