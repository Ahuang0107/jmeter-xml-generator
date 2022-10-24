use crate::elements::base::{
    bool, bool_prop, collection_prop, long_prop, obj_prop, string, string_prop, value,
};
use crate::script::ScriptElement;
use crate::xml::XmlEvent;

#[allow(dead_code)]
pub(crate) fn view_results_full_visualizer() -> ScriptElement {
    ScriptElement::from(
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
}

#[allow(dead_code)]
pub(crate) fn summary_report() -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("ResultCollector")
            .attr("guiclass", "SummaryReport")
            .attr("testclass", "ResultCollector")
            .attr("testname", "Summary Report")
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
}

#[allow(dead_code)]
pub(crate) fn perf_mon_collector(host_list: Vec<&str>) -> ScriptElement {
    ScriptElement::from(
        XmlEvent::start_element("kg.apc.jmeter.perfmon.PerfMonCollector")
            .attr("guiclass", "kg.apc.jmeter.vizualizers.PerfMonGui")
            .attr("testclass", "kg.apc.jmeter.perfmon.PerfMonCollector")
            .attr("testname", "jp@gc - PerfMon Metrics Collector")
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
            long_prop("interval_grouping", 1000),
            bool_prop("graph_aggregated", false),
            string_prop("include_sample_labels", ""),
            string_prop("exclude_sample_labels", ""),
            string_prop("start_offset", ""),
            string_prop("end_offset", ""),
            bool_prop("include_checkbox_state", false),
            bool_prop("exclude_checkbox_state", false),
            collection_prop("metricConnections", server_to_monitor(host_list)),
        ],
    )
}

fn server_to_monitor(host_list: Vec<&str>) -> Vec<ScriptElement> {
    let mut result = vec![];
    host_list.iter().for_each(|host| {
        result.push(collection_prop(
            "",
            vec![
                string_prop("", host),
                string_prop("1600768", "4444"),
                string_prop("66952", "CPU"),
                string_prop("0", ""),
            ],
        ));
        result.push(collection_prop(
            "",
            vec![
                string_prop("-1204607085", host),
                string_prop("1600768", "4444"),
                string_prop("-1993889503", "Memory"),
                string_prop("0", ""),
            ],
        ));
        result.push(collection_prop(
            "",
            vec![
                string_prop("", host),
                string_prop("1600768", "4444"),
                string_prop("2590131", "Swap"),
                string_prop("0", ""),
            ],
        ));
        result.push(collection_prop(
            "",
            vec![
                string_prop("", host),
                string_prop("1600768", "4444"),
                string_prop("2112896831", "Disks I/O"),
                string_prop("0", ""),
            ],
        ));
        result.push(collection_prop(
            "",
            vec![
                string_prop("", host),
                string_prop("1600768", "4444"),
                string_prop("-274342153", "Network I/O"),
                string_prop("0", ""),
            ],
        ));
    });
    result
}
