use crate::elements::base::{bool_prop, element_props, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

pub(crate) fn jmeter_test_plan(children: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("jmeterTestPlan")
            .attr("version", "1.2")
            .attr("properties", "5.0")
            .attr("jmeter", "5.5"),
        vec![ScriptElement::from_children(
            XmlEvent::start_element("hashTree"),
            children,
        )],
    )
}

pub(crate) fn test_plan(children: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("TestPlan")
            .attr("guiclass", "TestPlanGui")
            .attr("testclass", "TestPlan")
            .attr("testname", "Test Plan")
            .attr("enabled", "true"),
        vec![
            string_prop("TestPlan.comments", ""),
            bool_prop("TestPlan.functional_mode", false),
            bool_prop("TestPlan.tearDown_on_shutdown", true),
            bool_prop("TestPlan.serialize_threadgroups", false),
            element_props("TestPlan.user_defined_variables", vec![]),
            string_prop("TestPlan.user_define_classpath", ""),
        ],
    )
    .add_subs(children)
}
