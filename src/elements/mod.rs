use crate::elements::config::{
    arguments, cookie_manager, csv_data_set_config, header_manager, request_default,
};
use crate::elements::listeners::view_results_full_visualizer;
use crate::elements::logic_controller::once_only_controller;
use crate::elements::plan::{jmeter_test_plan, test_plan};
use crate::elements::request::http_sampler_proxy;
use crate::elements::threads::thread_group;
use crate::elements::timer::constant_throughput_timer;
use crate::generator::ScriptGenerator;
use crate::script::ScriptElement;

mod assertions;
mod base;
mod config;
mod listeners;
mod logic_controller;
mod plan;
mod post_processors;
mod pre_processors;
mod request;
mod threads;
mod timer;

pub fn root(gen: &ScriptGenerator) -> ScriptElement {
    let mut children: Vec<ScriptElement> = vec![];
    if !gen.once_requests.is_empty() {
        children.push(once_only_controller(
            gen.once_requests
                .iter()
                .map(|r| http_sampler_proxy(r))
                .collect(),
        ));
    }
    gen.data_sources.iter().for_each(|config| {
        children.push(csv_data_set_config(config));
    });
    if let Some(target_throughput) = gen.target_throughput {
        children.push(constant_throughput_timer(target_throughput * 60.0));
    }
    gen.requests
        .iter()
        .for_each(|r| children.push(http_sampler_proxy(r)));
    jmeter_test_plan(vec![test_plan(vec![
        arguments(&gen.variables),
        header_manager(&gen.headers),
        request_default(&gen.protocol, &gen.host, &gen.port),
        cookie_manager(),
        view_results_full_visualizer(),
        thread_group(
            gen.num_threads,
            gen.ramp_time,
            gen.loops,
            gen.lifetime,
            gen.stop_when_error,
            children,
        ),
    ])])
}
