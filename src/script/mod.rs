use std::io::Write;

use xml::EventWriter;
use xml::writer::events::StartElementBuilder;
use xml::writer::XmlEvent;

pub struct ScriptElement<'a> {
    start: StartElementBuilder<'a>,
    child: &'a str,
    children: Vec<ScriptElement<'a>>,
}

impl<'a> ScriptElement<'a> {
    #[allow(dead_code)]
    pub fn from(
        start: StartElementBuilder<'a>,
        children: Vec<ScriptElement<'a>>,
    ) -> ScriptElement<'a> {
        ScriptElement {
            start,
            child: "",
            children,
        }
    }
    #[allow(dead_code)]
    pub fn from_str(start: StartElementBuilder<'a>, child: &'a str) -> ScriptElement<'a> {
        ScriptElement {
            start,
            child,
            children: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn from_empty(start: StartElementBuilder<'a>) -> ScriptElement<'a> {
        ScriptElement {
            start,
            child: "",
            children: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn write<W: Write>(self, writer: &mut EventWriter<W>) {
        writer.write(self.start).expect("");
        writer.write(XmlEvent::characters(self.child)).expect("");
        self.children.into_iter().for_each(|c| c.write(writer));
        writer.write(XmlEvent::end_element()).expect("");
    }
}
