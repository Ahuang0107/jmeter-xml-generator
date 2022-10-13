use xml::writer::XmlEvent;

use crate::script::ScriptElement;

#[allow(dead_code)]
pub(crate) fn string_prop(name: &'static str, value: &'static str) -> ScriptElement<'static> {
    ScriptElement::from_str(
        XmlEvent::start_element("stringProp").attr("name", name),
        value,
    )
}

#[allow(dead_code)]
pub(crate) fn bool_prop(name: &'static str, value: bool) -> ScriptElement<'static> {
    ScriptElement::from_str(
        XmlEvent::start_element("boolProp").attr("name", name),
        if value { "true" } else { "false" },
    )
}

#[allow(dead_code)]
pub(crate) fn collection_prop(
    name: &'static str,
    value: Vec<ScriptElement<'static>>,
) -> ScriptElement<'static> {
    ScriptElement::from(
        XmlEvent::start_element("collectionProp").attr("name", name),
        value,
    )
}

#[allow(dead_code)]
pub(crate) fn element_prop(
    name: &'static str,
    e_type: &'static str,
    value: Vec<ScriptElement<'static>>,
) -> ScriptElement<'static> {
    ScriptElement::from(
        XmlEvent::start_element("elementProp")
            .attr("name", name)
            .attr("elementType", e_type),
        value,
    )
}

#[allow(dead_code)]
pub(crate) fn obj_prop(
    name: &'static str,
    value: ScriptElement<'static>,
) -> ScriptElement<'static> {
    ScriptElement::from(
        XmlEvent::start_element("objProp"),
        vec![
            ScriptElement::from_str(XmlEvent::start_element("name"), name),
            value,
        ],
    )
}

#[allow(dead_code)]
pub(crate) fn value(class: &'static str, v: Vec<ScriptElement<'static>>) -> ScriptElement<'static> {
    ScriptElement::from(XmlEvent::start_element("value").attr("class", class), v)
}

#[allow(dead_code)]
pub(crate) fn string(k: &'static str, v: &'static str) -> ScriptElement<'static> {
    ScriptElement::from_str(XmlEvent::start_element(k), v)
}

#[allow(dead_code)]
pub(crate) fn bool(k: &'static str, v: bool) -> ScriptElement<'static> {
    string(k, if v { "true" } else { "false" })
}
