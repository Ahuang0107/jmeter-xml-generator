use crate::script::ScriptElement;
use crate::xml::XmlEvent;

pub(crate) fn string_prop(name: &str, value: &str) -> ScriptElement {
    ScriptElement::from_str(
        XmlEvent::start_element("stringProp").attr("name", name),
        value,
    )
}

pub(crate) fn long_prop(name: &str, value: usize) -> ScriptElement {
    ScriptElement::from_str(
        XmlEvent::start_element("longProp").attr("name", name),
        value.to_string().as_str(),
    )
}

pub(crate) fn int_prop(name: &str, value: usize) -> ScriptElement {
    ScriptElement::from_str(
        XmlEvent::start_element("intProp").attr("name", name),
        value.to_string().as_str(),
    )
}

pub(crate) fn double_prop(name: &str, value: f32) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("doubleProp"),
        vec![
            ScriptElement::from_str(XmlEvent::start_element("name"), name),
            ScriptElement::from_str(XmlEvent::start_element("value"), value.to_string().as_str()),
            ScriptElement::from_str(XmlEvent::start_element("savedValue"), "0.0"),
        ],
    )
}

pub(crate) fn bool_prop(name: &str, value: bool) -> ScriptElement {
    ScriptElement::from_str(
        XmlEvent::start_element("boolProp").attr("name", name),
        if value { "true" } else { "false" },
    )
}

pub(crate) fn collection_prop(name: &str, value: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("collectionProp").attr("name", name),
        value,
    )
}

pub(crate) fn element_prop(name: &str, e_type: &str, value: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("elementProp")
            .attr("name", name)
            .attr("elementType", e_type),
        value,
    )
}

pub(crate) fn obj_prop(name: &str, value: ScriptElement) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("objProp"),
        vec![
            ScriptElement::from_str(XmlEvent::start_element("name"), name),
            value,
        ],
    )
}

pub(crate) fn value(class: &str, v: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from_children(XmlEvent::start_element("value").attr("class", class), v)
}

pub(crate) fn string(k: &str, v: &str) -> ScriptElement {
    ScriptElement::from_str(XmlEvent::start_element(k), v)
}

pub(crate) fn bool(k: &str, v: bool) -> ScriptElement {
    string(k, if v { "true" } else { "false" })
}

pub(crate) fn element_props(name: &str, args: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("elementProp")
            .attr("name", name)
            .attr("elementType", "Arguments")
            .attr("guiclass", "HTTPArgumentsPanel")
            .attr("testclass", "Arguments")
            .attr("testname", "User Defined Variables")
            .attr("enabled", "true"),
        vec![collection_prop("Arguments.arguments", args)],
    )
}

pub(crate) fn hash_tree(children: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from_children(XmlEvent::start_element("hashTree"), children)
}
