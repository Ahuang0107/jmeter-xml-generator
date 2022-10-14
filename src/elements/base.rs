use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub(crate) fn string_prop(name: String, value: String) -> ScriptElement {
    ScriptElement::from_str(
        XmlEvent::start_element("stringProp".to_string()).attr("name".to_string(), name),
        value,
    )
}

#[allow(dead_code)]
pub(crate) fn bool_prop(name: String, value: bool) -> ScriptElement {
    ScriptElement::from_str(
        XmlEvent::start_element("boolProp".to_string()).attr("name".to_string(), name),
        if value {
            "true".to_string()
        } else {
            "false".to_string()
        },
    )
}

#[allow(dead_code)]
pub(crate) fn collection_prop(name: String, value: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("collectionProp".to_string()).attr("name".to_string(), name),
        value,
    )
}

#[allow(dead_code)]
pub(crate) fn element_prop(
    name: String,
    e_type: String,
    value: Vec<ScriptElement>,
) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("elementProp".to_string())
            .attr("name".to_string(), name)
            .attr("elementType".to_string(), e_type),
        value,
    )
}

#[allow(dead_code)]
pub(crate) fn obj_prop(name: String, value: ScriptElement) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("objProp".to_string()),
        vec![
            ScriptElement::from_str(XmlEvent::start_element("name".to_string()), name),
            value,
        ],
    )
}

#[allow(dead_code)]
pub(crate) fn value(class: String, v: Vec<ScriptElement>) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("value".to_string()).attr("class".to_string(), class),
        v,
    )
}

#[allow(dead_code)]
pub(crate) fn string(k: String, v: String) -> ScriptElement {
    ScriptElement::from_str(XmlEvent::start_element(k), v)
}

#[allow(dead_code)]
pub(crate) fn bool(k: String, v: bool) -> ScriptElement {
    string(
        k,
        if v {
            "true".to_string()
        } else {
            "false".to_string()
        },
    )
}
