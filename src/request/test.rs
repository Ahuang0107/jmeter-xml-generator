use serde_json::json;

use crate::axios_config::AxiosRequestConfig;
use crate::request::{remove_json_string_variable_quota, RequestArg};

#[test]
fn check_request_arg() {
    let mut arg = RequestArg::new();
    arg.set_method("GET");
    arg.set_protocol("http");
    arg.set_host("127.0.0.1");
    arg.set_port("9090");
    arg.set_path("endpoint/login");
    assert_eq!(arg.method(), "GET");
    assert_eq!(arg.protocol(), "http");
    assert_eq!(arg.host(), "127.0.0.1");
    assert_eq!(arg.port(), "9090");
    assert_eq!(arg.path(), "endpoint/login");

    let mut arg = RequestArg::new();
    arg.set_method("GET");
    arg.set_protocol("https");
    arg.set_host("127.0.0.1");
    arg.set_path("endpoint/login");
    arg.set_header(("Content-Type", "application/json"));
    arg.set_param(json!({
        "username": "admin",
        "password": 123456,
        "rate": 5.5,
        "admin": true,
        "deleted": -1
    }));
    assert_eq!(arg.protocol(), "https");
    assert_eq!(arg.port(), "");
    assert_eq!(
        arg.headers(),
        &vec![(
            String::from("Content-Type"),
            String::from("application/json")
        )]
    );
    let mut mock_body = serde_json::Map::new();
    mock_body.insert(String::from("username"), json!("admin"));
    mock_body.insert(String::from("password"), json!(123456));
    mock_body.insert(String::from("rate"), json!(5.5));
    mock_body.insert(String::from("admin"), json!(true));
    mock_body.insert(String::from("deleted"), json!(-1));
    assert_eq!(arg.body(), serde_json::Value::Object(mock_body));
    assert!(!arg.if_form_body());
    assert!(!arg.if_json_body());
    arg.set_method("POST");
    assert!(!arg.if_form_body());
    assert!(arg.if_json_body());
    arg.clear_headers();
    arg.set_header(("Content-Type", "application/x-www-form-urlencoded"));
    assert!(arg.if_form_body());
    assert!(!arg.if_json_body());
    arg.set_method("PUT");
    assert!(!arg.if_form_body());
    assert!(arg.if_json_body());
    assert_eq!(
        arg.params(),
        vec![
            (String::from("admin"), String::from("true")),
            (String::from("deleted"), String::from("-1")),
            (String::from("password"), String::from("123456")),
            (String::from("rate"), String::from("5.5")),
            (String::from("username"), String::from("admin")),
        ]
    );
}

#[test]
fn check_from() {
    let config = serde_json::from_str::<AxiosRequestConfig>(GET).unwrap();
    let arg = RequestArg::from(config);
}

const GET: &str = r#"
{
  "headers": {
    "Accept": "application/json, text/plain, */*"
  },
  "baseURL": "http://127.0.0.1:9090/",
  "params": {
    "name": "elase"
  },
  "method": "get",
  "url": "/get"
}
"#;

#[test]
fn check_remove_quota() {
    let string = String::from(
        "{\"id\":\"${budgetId}\",\"name\":\"${budgetName}\",\"list\":${list},\"extension\":\"{}\"}",
    );
    let result = remove_json_string_variable_quota(string);
    assert_eq!(
        result,
        "{\"id\":${budgetId},\"name\":${budgetName},\"list\":${list},\"extension\":\"{}\"}"
    );
}
