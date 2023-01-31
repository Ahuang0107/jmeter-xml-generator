#![allow(dead_code)]
#![allow(unused_variables)]

mod axios_config;
mod elements;
pub mod generator;
#[cfg(target_arch = "wasm32")]
mod kit;
pub mod request;
mod script;
mod xml;
