use serde_json::json;

use crate::generator::ScriptGenerator;

#[test]
fn check_generator() {
    let mut generator = ScriptGenerator::new();
    generator.set_protocol("http");
    generator.set_host("127.0.0.1");
    generator.set_port("25000");
    generator.add_header("appCode", "resource");
    generator.add_header("test", "7EBZfzzrPWHBmXJtl#86LDs6varwXlYF");

    generator.once_post_form_data(
        "/endpoint/admin_user/login",
        Some(json!({
            "username":"smarthubdev",
            "password":"smarthub@1234"
        })),
    );

    generator.post(
        "endpoint/erp/budget/page",
        Some(json!({"status":0,"todoStatus":0,"current":1,"size":15})),
    );

    let target = generator.build();
    std::fs::create_dir_all("temp").unwrap_or_default();

    let path = String::from("temp/Test Plan - ")
        + &std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string()
        + ".jmx";

    let mut file = std::fs::File::create(path).unwrap();

    std::io::Write::write_all(&mut file, &target).unwrap();
}
