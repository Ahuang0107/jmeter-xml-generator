use crate::elements::base::{arguments, bool_prop, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub(crate) fn test_plan() -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("TestPlan".to_string())
            .attr("guiclass".to_string(), "TestPlanGui".to_string())
            .attr("testclass".to_string(), "TestPlan".to_string())
            .attr("testname".to_string(), "local test plan".to_string())
            .attr("enabled".to_string(), "true".to_string()),
        vec![
            string_prop("TestPlan.comments".to_string(), "".to_string()),
            bool_prop("TestPlan.functional_mode".to_string(), false),
            bool_prop("TestPlan.tearDown_on_shutdown".to_string(), true),
            bool_prop("TestPlan.serialize_threadgroups".to_string(), false),
            arguments("TestPlan.user_defined_variables".to_string(), vec![]),
            string_prop("TestPlan.user_define_classpath".to_string(), "".to_string()),
        ],
    )
}
