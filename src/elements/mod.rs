use crate::builder::Request;
use crate::elements::config::{config_test_element, cookie_manager, header_manager};
use crate::elements::http::http_sampler_proxy;
use crate::elements::listeners::result_collector;
use crate::elements::threads::thread_group;
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

mod base;
mod config;
mod http;
mod listeners;
mod threads;

#[allow(dead_code)]
pub fn root(
    host: String,
    port: String,
    headers: Vec<(String, String)>,
    requests: Vec<(Request, String)>,
) -> ScriptElement {
    let mut request_scripts: Vec<ScriptElement> = vec![];
    requests.into_iter().for_each(|(req, url)| {
        let with_json = req.is_post() && !req.with_form_data();
        request_scripts.push(http_sampler_proxy(req, url));
        if with_json {
            request_scripts.push(ScriptElement::from(
                XmlEvent::start_element("hashTree".to_string()),
                vec![header_manager(vec![(
                    "Content-Type".to_string(),
                    "application/json".to_string(),
                )])],
            ))
        } else {
            request_scripts.push(ScriptElement::from_empty(XmlEvent::start_element(
                "hashTree".to_string(),
            )))
        }
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
                ScriptElement::from_empty(XmlEvent::start_element("hashTree".to_string())),
                header_manager(headers),
                ScriptElement::from_empty(XmlEvent::start_element("hashTree".to_string())),
                cookie_manager(),
                ScriptElement::from_empty(XmlEvent::start_element("hashTree".to_string())),
                result_collector(),
                ScriptElement::from_empty(XmlEvent::start_element("hashTree".to_string())),
                test_plan(),
                ScriptElement::from(
                    XmlEvent::start_element("hashTree".to_string()),
                    vec![
                        thread_group(),
                        ScriptElement::from(
                            XmlEvent::start_element("hashTree".to_string()),
                            request_scripts,
                        ),
                    ],
                ),
            ],
        )],
    )
}

#[allow(dead_code)]
pub(crate) fn test_plan() -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("TestPlan".to_string())
            .attr("guiclass".to_string(), "TestPlanGui".to_string())
            .attr("testclass".to_string(), "TestPlan".to_string())
            .attr("testname".to_string(), "local test plan".to_string())
            .attr("enabled".to_string(), "true".to_string()),
        vec![
            ScriptElement::from_empty(
                XmlEvent::start_element("stringProp".to_string())
                    .attr("name".to_string(), "TestPlan.comments".to_string()),
            ),
            ScriptElement::from_str(
                XmlEvent::start_element("boolProp".to_string())
                    .attr("name".to_string(), "TestPlan.functional_mode".to_string()),
                "false".to_string(),
            ),
            ScriptElement::from_str(
                XmlEvent::start_element("boolProp".to_string()).attr(
                    "name".to_string(),
                    "TestPlan.tearDown_on_shutdown".to_string(),
                ),
                "true".to_string(),
            ),
            ScriptElement::from_str(
                XmlEvent::start_element("boolProp".to_string()).attr(
                    "name".to_string(),
                    "TestPlan.serialize_threadgroups".to_string(),
                ),
                "false".to_string(),
            ),
            ScriptElement::from(
                XmlEvent::start_element("elementProp".to_string())
                    .attr(
                        "name".to_string(),
                        "TestPlan.user_defined_variables".to_string(),
                    )
                    .attr("elementType".to_string(), "Arguments".to_string())
                    .attr("guiclass".to_string(), "ArgumentsPanel".to_string())
                    .attr("testclass".to_string(), "Arguments".to_string())
                    .attr("testname".to_string(), "User Defined Variables".to_string())
                    .attr("enabled".to_string(), "true".to_string()),
                vec![ScriptElement::from_empty(
                    XmlEvent::start_element("collectionProp".to_string())
                        .attr("name".to_string(), "Arguments.arguments".to_string()),
                )],
            ),
            ScriptElement::from_empty(XmlEvent::start_element("stringProp".to_string()).attr(
                "name".to_string(),
                "TestPlan.user_define_classpath".to_string(),
            )),
        ],
    )
}
