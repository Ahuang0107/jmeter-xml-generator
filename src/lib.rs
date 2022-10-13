use wasm_bindgen::prelude::*;

mod builder;
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

    use crate::builder::ScriptBuilder;

    #[test]
    fn test() {
        let mut script_builder = ScriptBuilder::new("192.168.206.112", "8080", 2);
        script_builder.add_header("appCode", "resource");
        script_builder.add_header("test", "7EBZfzzrPWHBmXJtl#86LDs6varwXlYF");
        let target = script_builder.build();

        let mut file = File::create("temp/file.jmx").expect("");

        file.write_all(&target).expect("");
    }
}
