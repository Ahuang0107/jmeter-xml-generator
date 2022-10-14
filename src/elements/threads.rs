use crate::elements::base::{bool_prop, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub(crate) fn thread_group() -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("ThreadGroup".to_string())
            .attr("guiclass".to_string(), "ThreadGroupGui".to_string())
            .attr("testclass".to_string(), "ThreadGroup".to_string())
            .attr("testname".to_string(), "first paint request".to_string())
            .attr("enabled".to_string(), "true".to_string()),
        vec![
            string_prop(
                "ThreadGroup.on_sample_error".to_string(),
                "continue".to_string(),
            ),
            ScriptElement::from(
                XmlEvent::start_element("elementProp".to_string())
                    .attr(
                        "name".to_string(),
                        "ThreadGroup.main_controller".to_string(),
                    )
                    .attr("elementType".to_string(), "LoopController".to_string())
                    .attr("guiclass".to_string(), "LoopControlPanel".to_string())
                    .attr("testclass".to_string(), "LoopController".to_string())
                    .attr("testname".to_string(), "Loop Controller".to_string())
                    .attr("enabled".to_string(), "true".to_string()),
                vec![
                    bool_prop("LoopController.continue_forever".to_string(), false),
                    string_prop("LoopController.loops".to_string(), "1".to_string()),
                ],
            ),
            string_prop("ThreadGroup.num_threads".to_string(), "1".to_string()),
            string_prop("ThreadGroup.ramp_time".to_string(), "2".to_string()),
            bool_prop("ThreadGroup.scheduler".to_string(), false),
            string_prop("ThreadGroup.delay".to_string(), "".to_string()),
            bool_prop("ThreadGroup.same_user_on_next_iteration".to_string(), true),
            string_prop("TestPlan.comments".to_string(), "".to_string()),
        ],
    )
}
