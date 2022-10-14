use crate::elements::base::{bool, bool_prop, obj_prop, string, string_prop, value};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub(crate) fn result_collector() -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("ResultCollector".to_string())
            .attr(
                "guiclass".to_string(),
                "ViewResultsFullVisualizer".to_string(),
            )
            .attr("testclass".to_string(), "ResultCollector".to_string())
            .attr("testname".to_string(), "View Results Tree".to_string())
            .attr("enabled".to_string(), "true".to_string()),
        vec![
            bool_prop("ResultCollector.error_logging".to_string(), false),
            obj_prop(
                "saveConfig".to_string(),
                value(
                    "SampleSaveConfiguration".to_string(),
                    vec![
                        bool("time".to_string(), true),
                        bool("latency".to_string(), true),
                        bool("timestamp".to_string(), true),
                        bool("success".to_string(), true),
                        bool("label".to_string(), true),
                        bool("code".to_string(), true),
                        bool("message".to_string(), true),
                        bool("threadName".to_string(), true),
                        bool("dataType".to_string(), true),
                        bool("encoding".to_string(), false),
                        bool("assertions".to_string(), true),
                        bool("subresults".to_string(), true),
                        bool("responseData".to_string(), false),
                        bool("samplerData".to_string(), false),
                        bool("xml".to_string(), false),
                        bool("fieldNames".to_string(), true),
                        bool("responseHeaders".to_string(), false),
                        bool("requestHeaders".to_string(), false),
                        bool("responseDataOnError".to_string(), false),
                        bool("saveAssertionResultsFailureMessage".to_string(), true),
                        string("assertionsResultsToSave".to_string(), "0".to_string()),
                        bool("bytes".to_string(), true),
                        bool("sentBytes".to_string(), true),
                        bool("url".to_string(), true),
                        bool("threadCounts".to_string(), true),
                        bool("idleTime".to_string(), true),
                        bool("connectTime".to_string(), true),
                    ],
                ),
            ),
            string_prop("filename".to_string(), "".to_string()),
        ],
    )
}
