use std::error::Error;
use std::io::Write;
use std::{fmt, io, result};

pub struct EventWriter {
    sink: Vec<u8>,
    emitter: Emitter,
}

impl EventWriter {
    pub fn new(sink: Vec<u8>) -> EventWriter {
        EventWriter {
            sink,
            emitter: Emitter::new(),
        }
    }

    pub fn write(&mut self, event: XmlEvent) -> Result<()> {
        match event {
            XmlEvent::StartDocument => self.emitter.emit_start_document(&mut self.sink),
            XmlEvent::StartElement { name, attributes } => {
                self.emitter
                    .emit_start_element(&mut self.sink, name, attributes)
            }
            XmlEvent::EndElement { name } => self.emitter.emit_end_element(&mut self.sink, name),
            XmlEvent::Characters(content) => self.emitter.emit_characters(&mut self.sink, content),
        }
    }

    pub fn export(self) -> Vec<u8> {
        self.sink
    }
}

pub struct Emitter {
    start_document_emitted: bool,
}

impl Emitter {
    pub fn new() -> Emitter {
        Emitter {
            start_document_emitted: false,
        }
    }
    pub fn emit_start_document(&mut self, target: &mut Vec<u8>) -> Result<()> {
        if self.start_document_emitted {
            return Err(EmitterError::DocumentStartAlreadyEmitted);
        }
        self.start_document_emitted = true;
        let result = {
            let mut write = move || {
                write!(target, "<?xml version=\"1.0\" encoding=\"utf-8\"?>")?;
                Ok(())
            };
            write()
        };
        result
    }
    pub fn emit_start_element(
        &mut self,
        target: &mut Vec<u8>,
        name: String,
        attributes: Vec<(String, String)>,
    ) -> Result<()> {
        if !self.start_document_emitted {
            self.emit_start_document(target)?;
        }
        write!(target, "<{}", name)?;
        for (k, v) in attributes.iter() {
            write!(target, " {}=\"{}\"", k, v)?
        }
        write!(target, ">")?;
        Ok(())
    }
    pub fn emit_end_element(&mut self, target: &mut Vec<u8>, name: Option<String>) -> Result<()> {
        match name {
            Some(actual) => {
                write!(target, "</{}>", actual)?;
            }
            None => {
                write!(target, "</>")?;
            }
        }
        Ok(())
    }
    pub fn emit_characters(&mut self, target: &mut Vec<u8>, content: String) -> Result<()> {
        target.write_all(content.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum EmitterError {
    Io(io::Error),
    DocumentStartAlreadyEmitted,
}

impl From<io::Error> for EmitterError {
    fn from(err: io::Error) -> EmitterError {
        EmitterError::Io(err)
    }
}

impl fmt::Display for EmitterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "emitter error: ")?;
        match *self {
            EmitterError::Io(ref e) => write!(f, "I/O error: {}", e),
            ref other => write!(f, "{}", other.to_string()),
        }
    }
}

impl Error for EmitterError {
    fn description(&self) -> &str {
        match *self {
            EmitterError::Io(_) => "I/O error",
            EmitterError::DocumentStartAlreadyEmitted => {
                "document start event has already been emitted"
            }
        }
    }
}

pub type Result<T> = result::Result<T, EmitterError>;

pub enum XmlEvent {
    #[allow(dead_code)]
    StartDocument,
    StartElement {
        name: String,
        attributes: Vec<(String, String)>,
    },
    EndElement {
        name: Option<String>,
    },
    Characters(String),
}

impl XmlEvent {
    pub fn start_element(name: String) -> StartElementBuilder {
        StartElementBuilder {
            name,
            attributes: Vec::new(),
        }
    }
    pub fn end_element(name: Option<String>) -> EndElementBuilder {
        EndElementBuilder { name }
    }
    pub fn characters(data: String) -> XmlEvent {
        XmlEvent::Characters(data)
    }
}

pub struct StartElementBuilder {
    name: String,
    attributes: Vec<(String, String)>,
}

impl StartElementBuilder {
    pub fn attr(mut self, name: String, value: String) -> StartElementBuilder {
        self.attributes.push((name, value));
        self
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

impl From<StartElementBuilder> for XmlEvent {
    #[inline]
    fn from(b: StartElementBuilder) -> Self {
        XmlEvent::StartElement {
            name: b.name,
            attributes: b.attributes,
        }
    }
}

pub struct EndElementBuilder {
    name: Option<String>,
}

impl From<EndElementBuilder> for XmlEvent {
    fn from(b: EndElementBuilder) -> Self {
        XmlEvent::EndElement { name: b.name }
    }
}
