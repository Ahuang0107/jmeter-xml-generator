mod builder;
mod elements;
mod script;
mod xml;

#[cfg(test)]
mod tests {
    use std::fs::{create_dir_all, File};
    use std::io::Write;

    use crate::builder::ScriptBuilder;

    #[test]
    fn test() {
        let mut script_builder = ScriptBuilder::mock();
        script_builder.add_header("appCode".to_string(), "resource".to_string());
        script_builder.add_header(
            "test".to_string(),
            "7EBZfzzrPWHBmXJtl#86LDs6varwXlYF".to_string(),
        );
        script_builder.add_axios_request(
            r#"
        {
            "baseUrl": "http://192.168.207.17/server/endpoint/",
            "url": "/admin_user/login",
            "method": "post",
            "heads": {"Content-Type":"multipart/form-data","appCode":"resource"},
            "params": "{}",
            "data": {"username":"smarthubdev","password":"smarthub@1234"}
        }"#
            .to_string(),
        );
        let target = script_builder.build();

        create_dir_all("temp").expect("fail to create directory temp");

        let mut file = File::create("temp/file.jmx".to_string()).expect("fail to create file.jmx");

        file.write_all(&target).expect("");
    }
}
