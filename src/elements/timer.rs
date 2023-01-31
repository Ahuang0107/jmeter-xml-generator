use crate::elements::base::{double_prop, int_prop, string_prop};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

pub(crate) fn constant_timer(delay: u128) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("ConstantTimer")
            .attr("guiclass", "ConstantTimerGui")
            .attr("testclass", "ConstantTimer")
            .attr("testname", "Constant Timer")
            .attr("enabled", "true"),
        vec![string_prop(
            "ConstantTimer.delay",
            delay.to_string().as_str(),
        )],
    )
    .with_subs(vec![])
}

pub(crate) fn constant_throughput_timer(throughput: f32) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("ConstantThroughputTimer")
            .attr("guiclass", "TestBeanGUI")
            .attr("testclass", "ConstantThroughputTimer")
            .attr("testname", "Constant Throughput Timer")
            .attr("enabled", "true"),
        vec![
            // 0 this thread only
            // 1 all active threads
            // 2 all active threads in current thread group
            int_prop("calcMode", 2),
            double_prop("throughput", throughput),
        ],
    )
    .with_subs(vec![])
}
