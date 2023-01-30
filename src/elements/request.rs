use crate::builder::{serialize_into_string_map_from_value, RequestArg};
use crate::elements::assertions::json_path_assertion;
use crate::elements::base::{bool_prop, element_prop, element_props, string_prop};
use crate::elements::config::{constant_timer, header_manager};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

impl RequestArg {
    pub fn with_form_data(&self) -> bool {
        self.heads
            .get("Content-Type")
            .unwrap_or(&"".to_string())
            .to_lowercase()
            .contains("multipart/form-data")
    }
    pub fn with_json(&self) -> bool {
        if self.method == "POST" || self.method == "PUT" {
            !self.with_form_data()
        } else {
            false
        }
    }
    fn data_hashmap(&self) -> std::collections::HashMap<String, String> {
        serialize_into_string_map_from_value(self.data.clone())
    }
    fn data_string(&self) -> String {
        self.data.clone().to_string()
    }
    pub fn generate_args(&self) -> Vec<ScriptElement> {
        return if self.method == "GET" {
            let result = self
                .params
                .clone()
                .into_iter()
                .map(|(k, v)| argument(k.as_str(), v.as_str()))
                .collect::<Vec<ScriptElement>>();
            return vec![element_props("HTTPsampler.Arguments", result)];
        } else if self.method == "POST" {
            if self.with_form_data() {
                let result = self
                    .data_hashmap()
                    .into_iter()
                    .map(|(k, v)| argument(k.as_str(), v.as_str()))
                    .collect::<Vec<ScriptElement>>();
                vec![element_props("HTTPsampler.Arguments", result)]
            } else {
                vec![
                    bool_prop("HTTPSampler.postBodyRaw", true),
                    element_props(
                        "HTTPsampler.Arguments",
                        vec![body_json(self.data_string().as_str())],
                    ),
                ]
            }
        } else if self.method == "PUT" {
            vec![
                bool_prop("HTTPSampler.postBodyRaw", true),
                element_props(
                    "HTTPsampler.Arguments",
                    vec![body_json(self.data_string().as_str())],
                ),
            ]
        } else {
            vec![]
        };
    }
}

#[allow(dead_code)]
pub fn http_sampler_proxy(request: &RequestArg) -> ScriptElement {
    let method = request.method.clone();
    let with_form_data = request.with_form_data();
    let path = request.path.clone();
    let delay = request.delay_time;
    let mut children = vec![
        string_prop("HTTPSampler.domain", request.host.as_str()),
        string_prop("HTTPSampler.port", request.port.as_str()),
        string_prop("HTTPSampler.protocol", request.protocol.as_str()),
        string_prop("HTTPSampler.contentEncoding", ""),
        string_prop("HTTPSampler.path", path.as_str()),
        string_prop("HTTPSampler.method", method.as_str()),
        bool_prop("HTTPSampler.follow_redirects", true),
        bool_prop("HTTPSampler.auto_redirects", false),
        bool_prop("HTTPSampler.use_keepalive", true),
        bool_prop("HTTPSampler.DO_MULTIPART_POST", with_form_data),
        string_prop("HTTPSampler.embedded_url_re", ""),
        string_prop("HTTPSampler.connect_timeout", ""),
        string_prop("HTTPSampler.response_timeout", ""),
    ];
    request
        .generate_args()
        .into_iter()
        .for_each(|g| children.push(g));
    let result = ScriptElement::from_children(
        XmlEvent::start_element("HTTPSamplerProxy")
            .attr("guiclass", "HttpTestSampleGui")
            .attr("testclass", "HTTPSamplerProxy")
            .attr("testname", path.as_str())
            .attr("enabled", "true"),
        children,
    );
    if request.with_json() {
        result.add_subs(vec![
            header_manager(vec![("Content-Type", "application/json")]),
            constant_timer(delay),
            // json_path_assertion("$.code", "200"),
        ])
    } else {
        result.add_subs(vec![
            constant_timer(delay),
            // json_path_assertion("$.code", "200"),
        ])
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
