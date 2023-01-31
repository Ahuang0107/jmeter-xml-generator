use crate::axios_config::AxiosRequestConfig;

#[test]
fn check_axios_request_config_serde_json() {
    let config = serde_json::from_str::<AxiosRequestConfig>(GET).unwrap();
    assert!(config.params.is_object());
    assert!(config.data.is_null());
    let config = serde_json::from_str::<AxiosRequestConfig>(POST).unwrap();
    assert!(config.params.is_null());
    assert!(config.data.is_object());
    let config = serde_json::from_str::<AxiosRequestConfig>(POST_FROM_DATA).unwrap();
    assert!(config.params.is_null());
    assert!(config.data.is_object());
    let config = serde_json::from_str::<AxiosRequestConfig>(PUT).unwrap();
    assert!(config.params.is_null());
    assert!(config.data.is_object());
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

const POST: &str = r#"
{
  "headers": {
    "Accept": "application/json, text/plain, */*"
  },
  "baseURL": "http://127.0.0.1:9090/",
  "method": "post",
  "url": "/post",
  "data": {
    "name": "elase"
  }
}
"#;

const POST_FROM_DATA: &str = r#"
{
  "headers": {
    "Accept": "application/json, text/plain, */*",
    "Content-Type": "multipart/form-data"
  },
  "baseURL": "http://127.0.0.1:9090/",
  "method": "post",
  "url": "/post-with-form",
  "data": {
    "name": "elase",
    "age": 24
  }
}
"#;

const PUT: &str = r#"
{
  "headers": {
    "Accept": "application/json, text/plain, */*"
  },
  "baseURL": "http://127.0.0.1:9090/",
  "method": "put",
  "url": "/put",
  "data": {
    "name": "elase"
  }
}
"#;
