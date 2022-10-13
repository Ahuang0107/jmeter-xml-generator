use xml::writer::XmlEvent;

use crate::elements::base::{bool_prop, string_prop};
use crate::script::ScriptElement;

#[allow(dead_code)]
pub(crate) fn thread_group<'a>() -> ScriptElement<'a> {
    ScriptElement::from(
        XmlEvent::start_element("ThreadGroup")
            .attr("guiclass", "ThreadGroupGui")
            .attr("testclass", "ThreadGroup")
            .attr("testname", "first paint request")
            .attr("enabled", "true"),
        vec![
            string_prop("ThreadGroup.on_sample_error", "continue"),
            ScriptElement::from(
                XmlEvent::start_element("elementProp")
                    .attr("name", "ThreadGroup.main_controller")
                    .attr("elementType", "LoopController")
                    .attr("guiclass", "LoopControlPanel")
                    .attr("testclass", "LoopController")
                    .attr("testname", "Loop Controller")
                    .attr("enabled", "true"),
                vec![
                    bool_prop("LoopController.continue_forever", false),
                    string_prop("LoopController.loops", "1"),
                ],
            ),
            string_prop("ThreadGroup.num_threads", "1"),
            string_prop("ThreadGroup.ramp_time", "2"),
            bool_prop("ThreadGroup.scheduler", false),
            string_prop("ThreadGroup.delay", ""),
            bool_prop("ThreadGroup.same_user_on_next_iteration", true),
            string_prop("TestPlan.comments", ""),
        ],
    )
}
