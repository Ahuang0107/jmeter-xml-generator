use crate::script::ScriptElement;
use crate::xml::XmlEvent;

pub(crate) fn once_only_controller(children: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("OnceOnlyController")
            .attr("guiclass", "OnceOnlyControllerGui")
            .attr("testclass", "OnceOnlyController")
            .attr("testname", "Once Only Controller")
            .attr("enabled", "true"),
        vec![],
    )
    .add_subs(children)
}
