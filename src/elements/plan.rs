use crate::elements::base::{arguments, bool_prop, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub(crate) fn test_plan() -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("TestPlan")
            .attr("guiclass", "TestPlanGui")
            .attr("testclass", "TestPlan")
            .attr("testname", "local test plan")
            .attr("enabled", "true"),
        vec![
            string_prop("TestPlan.comments", ""),
            bool_prop("TestPlan.functional_mode", false),
            bool_prop("TestPlan.tearDown_on_shutdown", true),
            bool_prop("TestPlan.serialize_threadgroups", false),
            arguments("TestPlan.user_defined_variables", vec![]),
            string_prop("TestPlan.user_define_classpath", ""),
        ],
    )
}
