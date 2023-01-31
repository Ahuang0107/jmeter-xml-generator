use crate::elements::assertions::json_path_assertion;
use crate::elements::base::{bool_prop, element_prop, element_props, string_prop};
use crate::elements::config::header_manager;
use crate::request::RequestArg;
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

pub fn http_sampler_proxy(request: &RequestArg) -> ScriptElement {
    let mut children = vec![
        string_prop("HTTPSampler.domain", request.host()),
        string_prop("HTTPSampler.port", request.port()),
        string_prop("HTTPSampler.protocol", request.protocol()),
        string_prop("HTTPSampler.contentEncoding", ""),
        string_prop("HTTPSampler.path", request.path()),
        string_prop("HTTPSampler.method", request.method()),
        bool_prop("HTTPSampler.follow_redirects", true),
        bool_prop("HTTPSampler.auto_redirects", false),
        bool_prop("HTTPSampler.use_keepalive", true),
        bool_prop("HTTPSampler.DO_MULTIPART_POST", request.if_form_data_body()),
        string_prop("HTTPSampler.embedded_url_re", ""),
        string_prop("HTTPSampler.connect_timeout", ""),
        string_prop("HTTPSampler.response_timeout", ""),
    ];
    match request.method() {
        "GET" => {
            vec![element_props(
                "HTTPsampler.Arguments",
                request
                    .params()
                    .iter()
                    .map(|(k, v)| argument(k, v))
                    .collect::<Vec<ScriptElement>>(),
            )]
        }
        "POST" => {
            if request.if_json_body() {
                vec![
                    bool_prop("HTTPSampler.postBodyRaw", true),
                    element_props(
                        "HTTPsampler.Arguments",
                        vec![body_json(&request.body_string())],
                    ),
                ]
            } else {
                vec![element_props(
                    "HTTPsampler.Arguments",
                    request
                        .params()
                        .iter()
                        .map(|(k, v)| argument(k, v))
                        .collect::<Vec<ScriptElement>>(),
                )]
            }
        }
        "PUT" => {
            vec![
                bool_prop("HTTPSampler.postBodyRaw", true),
                element_props(
                    "HTTPsampler.Arguments",
                    vec![body_json(&request.body_string())],
                ),
            ]
        }
        _ => {
            todo!()
        }
    }
    .into_iter()
    .for_each(|g| children.push(g));
    let result = ScriptElement::from_children(
        XmlEvent::start_element("HTTPSamplerProxy")
            .attr("guiclass", "HttpTestSampleGui")
            .attr("testclass", "HTTPSamplerProxy")
            .attr("testname", request.path())
            .attr("enabled", "true"),
        children,
    );
    if request.if_json_body() {
        result.add_subs(vec![
            header_manager(&vec![(
                String::from("Content-Type"),
                String::from("application/json"),
            )]),
            json_path_assertion("$.code", "0"),
        ])
    } else {
        result.add_subs(vec![json_path_assertion("$.code", "0")])
    }
}

fn argument(name: &str, value: &str) -> ScriptElement {
    element_prop(
        name,
        "HTTPArgument",
        vec![
            bool_prop("HTTPArgument.always_encode", true),
            string_prop("Argument.value", value),
            string_prop("Argument.metadata", "="),
            bool_prop("HTTPArgument.use_equals", true),
            string_prop("Argument.name", name),
        ],
    )
}

fn body_json(value: &str) -> ScriptElement {
    element_prop(
        "",
        "HTTPArgument",
        vec![
            bool_prop("HTTPArgument.always_encode", false),
            string_prop("Argument.value", value),
            string_prop("Argument.metadata", "="),
        ],
    )
}
