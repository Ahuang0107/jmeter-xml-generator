use crate::elements::base::{bool_prop, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

pub(crate) fn thread_group(
    num_threads: usize,
    ramp_time: usize,
    loops: Option<usize>,
    lifetime: Option<usize>,
    stop_when_error: bool,
    children: Vec<ScriptElement>,
) -> ScriptElement {
    let continue_forever = loops.is_none();
    let loops = if let Some(loops) = loops {
        loops.to_string()
    } else {
        String::from("-1")
    };
    let scheduler = lifetime.is_some();
    let lifetime = if let Some(lifetime) = lifetime {
        lifetime.to_string()
    } else {
        String::new()
    };
    ScriptElement::from_children(
        XmlEvent::start_element("ThreadGroup")
            .attr("guiclass", "ThreadGroupGui")
            .attr("testclass", "ThreadGroup")
            .attr("testname", "Thread Group")
            .attr("enabled", "true"),
        vec![
            // continue stopthread
            string_prop(
                "ThreadGroup.on_sample_error",
                if stop_when_error {
                    "stopthread"
                } else {
                    "continue"
                },
            ),
            ScriptElement::from_children(
                XmlEvent::start_element("elementProp")
                    .attr("name", "ThreadGroup.main_controller")
                    .attr("elementType", "LoopController")
                    .attr("guiclass", "LoopControlPanel")
                    .attr("testclass", "LoopController")
                    .attr("testname", "Loop Controller")
                    .attr("enabled", "true"),
                vec![
                    bool_prop("LoopController.continue_forever", continue_forever),
                    string_prop("LoopController.loops", &loops),
                ],
            ),
            string_prop("ThreadGroup.num_threads", &num_threads.to_string()),
            string_prop("ThreadGroup.ramp_time", &ramp_time.to_string()),
            bool_prop("ThreadGroup.scheduler", scheduler),
            string_prop("ThreadGroup.delay", ""),
            bool_prop("ThreadGroup.same_user_on_next_iteration", true),
            string_prop("ThreadGroup.duration", &lifetime),
        ],
    )
    .with_subs(children)
}
