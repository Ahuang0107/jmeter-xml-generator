use crate::elements::config::{arguments, cookie_manager, header_manager, request_default};
use crate::elements::listeners::view_results_full_visualizer;
use crate::elements::logic_controller::once_only_controller;
use crate::elements::plan::{jmeter_test_plan, test_plan};
use crate::elements::request::http_sampler_proxy;
use crate::elements::threads::thread_group;
use crate::request::RequestArg;
use crate::script::ScriptElement;

mod assertions;
mod base;
mod config;
mod listeners;
mod logic_controller;
mod plan;
mod request;
mod threads;

pub fn root(
    protocol: &str,
    host: &str,
    port: &str,
    variables: &Vec<(String, String)>,
    headers: &Vec<(String, String)>,
    once_requests: &Vec<RequestArg>,
    requests: &Vec<RequestArg>,
) -> ScriptElement {
    let mut children: Vec<ScriptElement> = vec![];
    children.push(once_only_controller(
        once_requests
            .into_iter()
            .map(|r| http_sampler_proxy(r))
            .collect(),
    ));
    requests
        .into_iter()
        .for_each(|r| children.push(http_sampler_proxy(r)));
    jmeter_test_plan(vec![test_plan(vec![
        arguments(variables),
        header_manager(headers),
        request_default(protocol, host, port),
        cookie_manager(),
        view_results_full_visualizer(),
        thread_group(children),
    ])])
}
