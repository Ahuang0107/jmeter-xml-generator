use crate::builder::serialize_into_string_map_from_str;

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
