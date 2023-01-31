use serde_json::json;

use jmeter_xml_generator::generator::{CsvDataSetConfig, ScriptGenerator};
use jmeter_xml_generator::request::RequestArg;

/// 示例：一段 登录，然后调用page接口获取数据，然后提交一份数据的 jmeter脚本生成
fn main() {
    let name = "script name";
    let mut gen = ScriptGenerator::new();
    gen.set_num_threads(10);
    gen.set_lifetime(Some(5));
    gen.set_stop_when_error(true);
    gen.set_loops(None); // 不设定循环次数，当csv数据循环完时终止
    gen.set_target_throughput(Some(200.0)); // 控制TPS在能达到的情况达到200per/s
    gen.set_protocol("http");
    gen.set_host("localhost");
    gen.set_port("9000");

    gen.add_data_source(CsvDataSetConfig::once("csv file path"));

    gen.once_request(RequestArg::from_post_form(
        "/login",
        Some(json!({
            "username":"${admin}",
            "password":"${password}"
        })),
    ));

    gen.request(
        RequestArg::from_get("/page", Some(json!({})))
            .with_json_processor(("resultData", "$.data.resultData")),
    );

    gen.request(RequestArg::from_post_form(
        "/submit",
        Some(json!({
            "resultData":"${resultData}"
        })),
    ));

    gen.generate_file_bundle(name);
}
