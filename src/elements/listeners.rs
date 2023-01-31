use crate::elements::base::{bool, bool_prop, obj_prop, string, string_prop, value};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub(crate) fn view_results_full_visualizer() -> ScriptElement {
    ScriptElement::from_children(
        XmlEvent::start_element("ResultCollector")
            .attr("guiclass", "ViewResultsFullVisualizer")
            .attr("testclass", "ResultCollector")
            .attr("testname", "View Results Tree")
            .attr("enabled", "true"),
        vec![
            bool_prop("ResultCollector.error_logging", false),
            obj_prop(
                "saveConfig",
                value(
                    "SampleSaveConfiguration",
                    vec![
                        bool("time", true),
                        bool("latency", true),
                        bool("timestamp", true),
                        bool("success", true),
                        bool("label", true),
                        bool("code", true),
                        bool("message", true),
                        bool("threadName", true),
                        bool("dataType", true),
                        bool("encoding", false),
                        bool("assertions", true),
                        bool("subresults", true),
                        bool("responseData", false),
                        bool("samplerData", false),
                        bool("xml", false),
                        bool("fieldNames", true),
                        bool("responseHeaders", false),
                        bool("requestHeaders", false),
                        bool("responseDataOnError", false),
                        bool("saveAssertionResultsFailureMessage", true),
                        string("assertionsResultsToSave", "0"),
                        bool("bytes", true),
                        bool("sentBytes", true),
                        bool("url", true),
                        bool("threadCounts", true),
                        bool("idleTime", true),
                        bool("connectTime", true),
                    ],
                ),
            ),
            string_prop("filename", ""),
        ],
    )
    .with_subs(vec![])
}
