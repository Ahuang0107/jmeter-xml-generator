use xml::writer::XmlEvent;

use crate::script::ScriptElement;

#[allow(dead_code)]
pub(crate) fn string_prop<'a>(name: &'a str, value: &'a str) -> ScriptElement<'a> {
    ScriptElement::from_str(
        XmlEvent::start_element("stringProp").attr("name", name),
        value,
    )
}

#[allow(dead_code)]
pub(crate) fn bool_prop(name: &str, value: bool) -> ScriptElement {
    ScriptElement::from_str(
        XmlEvent::start_element("boolProp").attr("name", name),
        if value { "true" } else { "false" },
    )
}

#[allow(dead_code)]
pub(crate) fn collection_prop<'a>(
    name: &'a str,
    value: Vec<ScriptElement<'a>>,
) -> ScriptElement<'a> {
    ScriptElement::from(
        XmlEvent::start_element("collectionProp").attr("name", name),
        value,
    )
}

#[allow(dead_code)]
pub(crate) fn element_prop<'a>(
    name: &'a str,
    e_type: &'a str,
    value: Vec<ScriptElement<'a>>,
) -> ScriptElement<'a> {
    ScriptElement::from(
        XmlEvent::start_element("elementProp")
            .attr("name", name)
            .attr("elementType", e_type),
        value,
    )
}

#[allow(dead_code)]
pub(crate) fn obj_prop<'a>(name: &'a str, value: ScriptElement<'a>) -> ScriptElement<'a> {
    ScriptElement::from(
        XmlEvent::start_element("objProp"),
        vec![
            ScriptElement::from_str(XmlEvent::start_element("name"), name),
            value,
        ],
    )
}

#[allow(dead_code)]
pub(crate) fn value<'a>(class: &'a str, v: Vec<ScriptElement<'a>>) -> ScriptElement<'a> {
    ScriptElement::from(XmlEvent::start_element("value").attr("class", class), v)
}

#[allow(dead_code)]
pub(crate) fn string<'a>(k: &'a str, v: &'a str) -> ScriptElement<'a> {
    ScriptElement::from_str(XmlEvent::start_element(k), v)
}

#[allow(dead_code)]
pub(crate) fn bool(k: &str, v: bool) -> ScriptElement {
    string(k, if v { "true" } else { "false" })
}
