use xml::writer::XmlEvent;

use crate::elements::config::{config_test_element, cookie_manager, header_manager};
use crate::elements::http::http_sampler_proxy;
pub(crate) use crate::elements::http::Request;
use crate::elements::listeners::result_collector;
use crate::elements::threads::thread_group;
use crate::script::ScriptElement;

mod base;
mod config;
mod http;
mod listeners;
mod threads;

#[allow(dead_code)]
pub fn root<'a>(
    host: &'a str,
    port: &'a str,
    headers: Vec<(&'a str, &'a str)>,
    requests: Vec<Request<'a>>,
) -> ScriptElement<'a> {
    let mut request_scripts: Vec<ScriptElement> = vec![];
    requests.into_iter().for_each(|req| {
        let with_json = req.with_json();
        request_scripts.push(http_sampler_proxy(req));
        if with_json {
            request_scripts.push(ScriptElement::from(
                XmlEvent::start_element("hashTree"),
                vec![header_manager(vec![("Content-Type", "application/json")])],
            ))
        } else {
            request_scripts.push(ScriptElement::from_empty(XmlEvent::start_element(
                "hashTree",
            )))
        }
    });
    ScriptElement::from(
        XmlEvent::start_element("jmeterTestPlan")
            .attr("version", "1.2")
            .attr("properties", "5.0")
            .attr("jmeter", "5.5"),
        vec![ScriptElement::from(
            XmlEvent::start_element("hashTree"),
            vec![
                config_test_element(host, port),
                ScriptElement::from_empty(XmlEvent::start_element("hashTree")),
                header_manager(headers),
                ScriptElement::from_empty(XmlEvent::start_element("hashTree")),
                cookie_manager(),
                ScriptElement::from_empty(XmlEvent::start_element("hashTree")),
                result_collector(),
                ScriptElement::from_empty(XmlEvent::start_element("hashTree")),
                test_plan(),
                ScriptElement::from(
                    XmlEvent::start_element("hashTree"),
                    vec![
                        thread_group(),
                        ScriptElement::from(XmlEvent::start_element("hashTree"), request_scripts),
                    ],
                ),
            ],
        )],
    )
}

#[allow(dead_code)]
pub(crate) fn test_plan() -> ScriptElement<'static> {
    ScriptElement::from(
        XmlEvent::start_element("TestPlan")
            .attr("guiclass", "TestPlanGui")
            .attr("testclass", "TestPlan")
            .attr("testname", "local test plan")
            .attr("enabled", "true"),
        vec![
            ScriptElement::from_empty(
                XmlEvent::start_element("stringProp").attr("name", "TestPlan.comments"),
            ),
            ScriptElement::from_str(
                XmlEvent::start_element("boolProp").attr("name", "TestPlan.functional_mode"),
                "false",
            ),
            ScriptElement::from_str(
                XmlEvent::start_element("boolProp").attr("name", "TestPlan.tearDown_on_shutdown"),
                "true",
            ),
            ScriptElement::from_str(
                XmlEvent::start_element("boolProp").attr("name", "TestPlan.serialize_threadgroups"),
                "false",
            ),
            ScriptElement::from(
                XmlEvent::start_element("elementProp")
                    .attr("name", "TestPlan.user_defined_variables")
                    .attr("elementType", "Arguments")
                    .attr("guiclass", "ArgumentsPanel")
                    .attr("testclass", "Arguments")
                    .attr("testname", "User Defined Variables")
                    .attr("enabled", "true"),
                vec![ScriptElement::from_empty(
                    XmlEvent::start_element("collectionProp").attr("name", "Arguments.arguments"),
                )],
            ),
            ScriptElement::from_empty(
                XmlEvent::start_element("stringProp")
                    .attr("name", "TestPlan.user_define_classpath"),
            ),
        ],
    )
}
