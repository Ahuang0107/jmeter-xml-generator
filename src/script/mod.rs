use crate::xml::{EventWriter, StartElementBuilder, XmlEvent};

pub struct ScriptElement {
    start: StartElementBuilder,
    child: String,
    children: Vec<ScriptElement>,
    sub_elements: Option<Vec<ScriptElement>>,
}

impl ScriptElement {
    pub fn from_children(
        start: StartElementBuilder,
        children: Vec<ScriptElement>,
    ) -> ScriptElement {
        ScriptElement {
            start,
            child: "".to_string(),
            children,
            sub_elements: None,
        }
    }
    pub fn from_str(start: StartElementBuilder, child: &str) -> ScriptElement {
        ScriptElement {
            start,
            child: child.to_string(),
            children: Vec::new(),
            sub_elements: None,
        }
    }
    pub fn add_subs(&mut self, subs: Vec<ScriptElement>) {
        if let Some(ref mut els) = self.sub_elements {
            els.extend(subs);
        } else {
            self.sub_elements = Some(subs)
        }
    }
    pub fn with_subs(mut self, subs: Vec<ScriptElement>) -> ScriptElement {
        self.add_subs(subs);
        self
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
        match self.sub_elements {
            None => {}
            Some(els) => {
                writer
                    .write(XmlEvent::start_element("hashTree").into())
                    .expect("");
                els.into_iter().for_each(|c| c.write(writer));
                writer
                    .write(XmlEvent::end_element(Some("hashTree".to_string())).into())
                    .expect("");
            }
        }
    }
}
