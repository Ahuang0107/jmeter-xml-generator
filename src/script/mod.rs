use crate::xml::{EventWriter, StartElementBuilder, XmlEvent};

pub struct ScriptElement {
    start: StartElementBuilder,
    child: String,
    children: Vec<ScriptElement>,
}

impl ScriptElement {
    #[allow(dead_code)]
    pub fn from(start: StartElementBuilder, children: Vec<ScriptElement>) -> ScriptElement {
        ScriptElement {
            start,
            child: "".to_string(),
            children,
        }
    }
    #[allow(dead_code)]
    pub fn from_str(start: StartElementBuilder, child: &str) -> ScriptElement {
        ScriptElement {
            start,
            child: child.to_string(),
            children: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn from_empty(start: StartElementBuilder) -> ScriptElement {
        ScriptElement {
            start,
            child: "".to_string(),
            children: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn write(self, writer: &mut EventWriter) {
        let name = self.start.name();
        writer.write(self.start.into()).expect("");
        writer.write(XmlEvent::characters(self.child)).expect("");
        self.children.into_iter().for_each(|c| c.write(writer));
        writer
            .write(XmlEvent::end_element(Some(name)).into())
            .expect("");
    }
}
