use crate::builder::RequestArg;

impl Default for RequestArg {
    fn default() -> Self {
        Self {
            protocol: "http".to_string(),
            host: "localhost".to_string(),
            port: "8000".to_string(),
            method: "GET".to_string(),
            path: "/login".to_string(),
            heads: std::collections::HashMap::new(),
            params: std::collections::HashMap::new(),
            data: serde_json::Value::default(),
            delay_time: 0,
        }
    }
}

#[test]
fn test_with_form_data() {
    let mut arg = RequestArg::default();
    assert_eq!(arg.with_form_data(), false);
    arg.heads.insert(
        "Content-Type".to_string(),
        "multipart/form-data".to_string(),
    );
    assert_eq!(arg.with_form_data(), true);
}

#[test]
fn test_with_json() {
    let mut arg = RequestArg::default();
    assert_eq!(arg.with_json(), false);
    arg.method = "PUT".to_string();
    assert_eq!(arg.with_json(), true);
    arg.method = "POST".to_string();
    assert_eq!(arg.with_json(), true);
    arg.heads.insert(
        "Content-Type".to_string(),
        "multipart/form-data".to_string(),
    );
    assert_eq!(arg.with_json(), false);
}
