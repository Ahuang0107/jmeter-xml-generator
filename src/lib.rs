use wasm_bindgen::prelude::*;

mod builder;
mod elements;
mod script;
mod xml;

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
    use std::fs::File;
    use std::io::Write;

    use crate::builder::ScriptBuilder;
    use crate::elements::Request;

    #[test]
    fn test() {
        let mut script_builder =
            ScriptBuilder::new("192.168.206.112".to_string(), "8080".to_string(), 2);
        script_builder.add_header("appCode".to_string(), "resource".to_string());
        script_builder.add_header(
            "test".to_string(),
            "7EBZfzzrPWHBmXJtl#86LDs6varwXlYF".to_string(),
        );
        script_builder.add_request(Request::from(
            "/endpoint/admin_user/login".to_string(),
            "post".to_string(),
            true,
            vec![
                ("username".to_string(), "smarthubdev".to_string()),
                ("password".to_string(), "smarthub@1234".to_string()),
            ],
            None,
        ));
        script_builder.add_request(Request::from(
            "/endpoint/basic/data_dictionary/bu_list".to_string(),
            "get".to_string(),
            false,
            vec![],
            None,
        ));
        script_builder.add_request(Request::from(
            "/endpoint/basic/data_dictionary/list".to_string(),
            "get".to_string(),
            false,
            vec![("type".to_string(), "4".to_string())],
            None,
        ));
        script_builder.add_request(Request::from(
            "/endpoint/erp/budget/page".to_string(),
            "post".to_string(),
            false,
            vec![],
            Some("{\"current\":1,\"size\":15,\"status\":0}".to_string()),
        ));
        let target = script_builder.build();

        let mut file = File::create("temp/file.jmx".to_string()).expect("");

        file.write_all(&target).expect("");
    }
}
