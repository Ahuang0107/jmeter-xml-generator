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
    use crate::elements::Request;

    #[test]
    fn test() {
        let mut script_builder = ScriptBuilder::new("192.168.206.112", "8080", 2);
        script_builder.add_header("appCode", "resource");
        script_builder.add_header("test", "7EBZfzzrPWHBmXJtl#86LDs6varwXlYF");
        script_builder.add_request(Request::from(
            "/endpoint/admin_user/login",
            "post",
            true,
            vec![("username", "smarthubdev"), ("password", "smarthub@1234")],
            None,
        ));
        script_builder.add_request(Request::from(
            "/endpoint/basic/data_dictionary/bu_list",
            "get",
            false,
            vec![],
            None,
        ));
        script_builder.add_request(Request::from(
            "/endpoint/basic/data_dictionary/list",
            "get",
            false,
            vec![("type", "4")],
            None,
        ));
        script_builder.add_request(Request::from(
            "/endpoint/erp/budget/page",
            "post",
            false,
            vec![],
            Some("{\"current\":1,\"size\":15,\"status\":0}"),
        ));
        let target = script_builder.build();

        let mut file = File::create("temp/file.jmx").expect("");

        file.write_all(&target).expect("");
    }
}
