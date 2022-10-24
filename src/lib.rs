use wasm_bindgen::prelude::*;

mod builder;
mod elements;
mod script;
mod utils;
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
    use crate::utils::KeyValue;

    #[test]
    fn test() {
        let mut script_builder = ScriptBuilder::new(
            "192.168.206.112".to_string(),
            "8080".to_string(),
            500,
            1,
            serde_json::to_string::<Vec<String>>(&vec![
                "CNSHAVMPSMHAP01.ey.net".to_string(),
                "CNSHAVMPSMHAP02.ey.net".to_string(),
                "CNSHAVMPSMHDB01.ey.net".to_string(),
            ])
            .unwrap(),
        );
        script_builder.add_header("appCode".to_string(), "resource".to_string());
        script_builder.add_header(
            "test".to_string(),
            "7EBZfzzrPWHBmXJtl#86LDs6varwXlYF".to_string(),
        );
        script_builder.post_with_form_data(
            "/endpoint/admin_user/login".to_string(),
            serde_json::to_string::<Vec<KeyValue>>(&vec![
                KeyValue::from("username".to_string(), "smarthubdev".to_string()),
                KeyValue::from("password".to_string(), "smarthub".to_string()),
            ])
            .unwrap(),
            0,
        );
        script_builder.get(
            "/endpoint/basic/data_dictionary/bu_list".to_string(),
            serde_json::to_string::<Vec<KeyValue>>(&vec![]).unwrap(),
            0,
        );
        script_builder.get(
            "/endpoint/basic/data_dictionary/list".to_string(),
            serde_json::to_string::<Vec<KeyValue>>(&vec![KeyValue::from(
                "type".to_string(),
                "4".to_string(),
            )])
            .unwrap(),
            0,
        );
        script_builder.post(
            "/endpoint/erp/budget/page".to_string(),
            "{\"current\":1,\"size\":15,\"status\":0}".to_string(),
            1000,
        );
        script_builder.get(
            "/endpoint/staffing/page".to_string(),
            serde_json::to_string::<Vec<KeyValue>>(&vec![
                KeyValue::from("financeYear".to_string(), "2023".to_string()),
                KeyValue::from("current".to_string(), "1".to_string()),
                KeyValue::from("size".to_string(), "15".to_string()),
            ])
            .unwrap(),
            1000,
        );
        script_builder.get(
            "/endpoint/staffing/12/detail".to_string(),
            serde_json::to_string::<Vec<KeyValue>>(&vec![]).unwrap(),
            1000,
        );
        script_builder.put(
            "/endpoint/staffing/12".to_string(),
            "{\"stage\":\"PENDING_RMA_REVIEW\"}".to_string(),
            1000,
        );
        let target = script_builder.build();

        let mut file = File::create("temp/file.jmx".to_string()).expect("");

        file.write_all(&target).expect("");
    }
}
