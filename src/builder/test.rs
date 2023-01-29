use crate::builder::{serialize_into_string_map_from_str, AxiosConfig};

#[test]
fn test_serialize_into_string_map_from_str() {
    let result = serialize_into_string_map_from_str(
        "{\"string\":\"string value\",\"number\":123,\"boolean\":true,\"null\":null,\"object\":{\"one\":1,\"two\":2},\"array\":[\"one\",\"two\"]}",
    );
    assert_eq!(result.get("string"), Some(&"string value".to_string()));
    assert_eq!(result.get("number"), Some(&"123".to_string()));
    assert_eq!(result.get("boolean"), Some(&"true".to_string()));
    assert_eq!(result.get("null"), Some(&"null".to_string()));
    assert_eq!(result.get("undefined"), None);
    assert_eq!(
        result.get("object"),
        Some(&"{\"one\":1,\"two\":2}".to_string())
    );
    assert_eq!(result.get("array"), Some(&"[\"one\",\"two\"]".to_string()));
    let result = std::panic::catch_unwind(|| {
        serialize_into_string_map_from_str("{\"string\":,\"number\":123,\"boolean\":true}");
    });
    assert!(result.is_err());
    let result = serialize_into_string_map_from_str("[\"one\",\"two\"]");
    assert!(result.is_empty());
    let result = serialize_into_string_map_from_str(
        "{\"Content-Type\":\"multipart/form-data\",\"appCode\":\"resource\"}",
    );
    assert_eq!(
        result.get("Content-Type"),
        Some(&"multipart/form-data".to_string())
    );
}

#[test]
fn test_into_request_arg() {
    let config = AxiosConfig {
        base_url: "http://localhost/endpoint/".to_string(),
        url: "/eic_booking/listByEngagementCodeIds?startDate=1643677207051&endDate=1706662807051&engagementCodeIds=22920".to_string(),
        method: "GET".to_string(),
        heads: std::collections::HashMap::new(),
        params: std::collections::HashMap::new(),
        data: serde_json::Value::Null,
    };
    let arg = config.into_request_arg(0);
    assert_eq!(arg.params.len(), 3);
    assert_eq!(
        arg.params.get("startDate"),
        Some(&"1643677207051".to_string())
    );
}

#[test]
fn test() {
    let mut script_builder = crate::builder::ScriptGenerator::new();
    script_builder.switch = true;
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

    std::fs::create_dir_all("temp").expect("fail to create directory temp");

    let mut file =
        std::fs::File::create("temp/file.jmx".to_string()).expect("fail to create file.jmx");

    std::io::Write::write_all(&mut file, &target).expect("");
}
