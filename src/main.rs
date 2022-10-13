extern crate xml;

use std::fs::File;
use std::io::Write;

use xml::writer::XmlEvent;
use xml::EmitterConfig;

use crate::script::ScriptElement;

mod script;

fn main() {
    let mut target: Vec<u8> = Vec::new();

    let mut writer = EmitterConfig::new().create_writer(&mut target);
    let script = ScriptElement::from(
        XmlEvent::start_element("jmeterTestPlan").attr("version", "1.2"),
        vec![ScriptElement::from(
            XmlEvent::start_element("hashTree"),
            vec![ScriptElement::from(
                XmlEvent::start_element("TestPlan").attr("guiclass", "TestPlanGui"),
                vec![ScriptElement::from_str(
                    XmlEvent::start_element("stringProp"),
                    "192.168.206.112",
                )],
            )],
        )],
    );

    script.write(&mut writer);

    let mut file = File::create("file.jmx").expect("");

    file.write_all(&target).expect("");
}
