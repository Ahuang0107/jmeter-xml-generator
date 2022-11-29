use crate::builder::RequestArg;
use crate::elements::config::{arguments, cookie_manager, header_manager};
use crate::elements::listeners::view_results_full_visualizer;
use crate::elements::plan::{jmeter_test_plan, test_plan};
use crate::elements::request::http_sampler_proxy;
use crate::elements::threads::thread_group;
use crate::script::ScriptElement;

mod base;
mod config;
mod listeners;
mod plan;
mod request;
#[cfg(test)]
mod test;
mod threads;

#[allow(dead_code)]
pub fn root(
    variables: std::collections::HashMap<&str, &str>,
    headers: Vec<(&str, &str)>,
    requests: Vec<RequestArg>,
) -> ScriptElement {
    jmeter_test_plan(vec![test_plan(vec![
        arguments(variables),
        header_manager(headers),
        cookie_manager(),
        view_results_full_visualizer(),
        thread_group(
            requests
                .into_iter()
                .map(|r| http_sampler_proxy(&r))
                .collect(),
        ),
    ])])
}
