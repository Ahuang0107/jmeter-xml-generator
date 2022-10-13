use wasm_bindgen::prelude::*;

mod elements;
mod script;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn hello_world() {
    console_log!("Hello World!");
}

#[cfg(test)]
mod tests {
    extern crate xml;

    use std::fs::File;
    use std::io::Write;

    use xml::EmitterConfig;

    use crate::elements::root_element;

    #[test]
    fn test() {
        let mut target: Vec<u8> = Vec::new();

        let mut writer = EmitterConfig::new().create_writer(&mut target);
        let script = root_element();

        script.write(&mut writer);

        let mut file = File::create("temp/file.jmx").expect("");

        file.write_all(&target).expect("");
    }
}
