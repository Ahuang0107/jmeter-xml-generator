use crate::builder::Request;
use crate::elements::base::hash_tree;
use crate::elements::config::{
    config_test_element, constant_timer, cookie_manager, header_manager,
};
use crate::elements::http::http_sampler_proxy;
use crate::elements::listeners::result_collector;
use crate::elements::plan::test_plan;
use crate::elements::threads::thread_group;
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

mod base;
mod config;
mod http;
mod listeners;
mod plan;
mod threads;

#[allow(dead_code)]
pub fn root(
    host: String,
    port: String,
    headers: Vec<(String, String)>,
    requests: Vec<(Request, String, usize)>,
) -> ScriptElement {
    let mut request_scripts: Vec<ScriptElement> = vec![];
    requests.into_iter().for_each(|(req, url, delay_time)| {
        let with_json = (req.is_post() || req.is_put()) && !req.with_form_data();
        request_scripts.push(http_sampler_proxy(req, url));
        let mut hash_tree_children: Vec<ScriptElement> = vec![];
        if with_json {
            hash_tree_children.push(header_manager(vec![(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )]));
            hash_tree_children.push(hash_tree(vec![]));
        }
        if delay_time > 0 {
            hash_tree_children.push(constant_timer(delay_time));
            hash_tree_children.push(hash_tree(vec![]));
        }
        request_scripts.push(hash_tree(hash_tree_children));
    });
    ScriptElement::from(
        XmlEvent::start_element("jmeterTestPlan".to_string())
            .attr("version".to_string(), "1.2".to_string())
            .attr("properties".to_string(), "5.0".to_string())
            .attr("jmeter".to_string(), "5.5".to_string()),
        vec![ScriptElement::from(
            XmlEvent::start_element("hashTree".to_string()),
            vec![
                config_test_element(host, port),
                hash_tree(vec![]),
                header_manager(headers),
                hash_tree(vec![]),
                cookie_manager(),
                hash_tree(vec![]),
                result_collector(),
                hash_tree(vec![]),
                test_plan(),
                hash_tree(vec![thread_group(), hash_tree(request_scripts)]),
            ],
        )],
    )
}
