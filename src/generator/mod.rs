use crate::elements::root;
use crate::request::RequestArg;
use crate::xml::EventWriter;

pub struct ScriptGenerator {
    // User Defined Variables
    pub(crate) variables: Vec<(String, String)>,
    // HTTP Header Manager
    pub(crate) headers: Vec<(String, String)>,
    // HTTP Request Default
    pub(crate) protocol: String,
    pub(crate) host: String,
    pub(crate) port: String,
    pub(crate) num_threads: usize,
    pub(crate) ramp_time: usize,
    pub(crate) loops: Option<usize>,
    pub(crate) lifetime: Option<usize>,
    pub(crate) stop_when_error: bool,
    /// target TPS
    pub(crate) target_throughput: Option<f32>,
    pub(crate) once_requests: Vec<RequestArg>,
    pub(crate) requests: Vec<RequestArg>,
    pub(crate) data_sources: Vec<CsvDataSetConfig>,
    pub(crate) timer: instant::Instant,
}

pub struct CsvDataSetConfig {
    pub(crate) filename: String,
    pub(crate) recycle: bool,
    pub(crate) stop_thread: bool,
}

impl CsvDataSetConfig {
    pub fn recycle(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            recycle: true,
            stop_thread: false,
        }
    }
    pub fn once(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            recycle: false,
            stop_thread: true,
        }
    }
}

impl ScriptGenerator {
    pub fn new() -> ScriptGenerator {
        ScriptGenerator {
            variables: vec![],
            headers: vec![],
            protocol: String::new(),
            host: String::new(),
            port: String::new(),
            num_threads: 1,
            ramp_time: 1,
            loops: Some(1),
            lifetime: None,
            stop_when_error: false,
            target_throughput: None,
            once_requests: Vec::new(),
            requests: Vec::new(),
            data_sources: vec![],
            timer: instant::Instant::now(),
        }
    }
    pub fn set_num_threads(&mut self, num_threads: usize) {
        self.num_threads = num_threads;
    }
    // pub fn set_ramp_time(&mut self, ramp_time: usize) {
    //     self.ramp_time = ramp_time;
    // }
    pub fn set_loops(&mut self, loops: Option<usize>) {
        self.loops = loops;
    }
    pub fn set_lifetime(&mut self, lifetime: Option<usize>) {
        self.lifetime = lifetime;
    }
    pub fn set_stop_when_error(&mut self, value: bool) {
        self.stop_when_error = value
    }
    pub fn set_target_throughput(&mut self, target_throughput: Option<f32>) {
        self.target_throughput = target_throughput;
    }
    pub fn set_protocol(&mut self, protocol: &str) {
        self.protocol = protocol.to_string();
    }
    pub fn set_host(&mut self, host: &str) {
        self.host = host.to_string();
    }
    pub fn set_port(&mut self, port: &str) {
        self.port = port.to_string();
    }
    pub fn add_header(&mut self, k: &str, v: &str) {
        self.headers.push((k.to_string(), v.to_string()));
    }
    pub fn request(&mut self, request: RequestArg) {
        self.requests.push(request)
    }
    pub fn once_request(&mut self, request: RequestArg) {
        self.once_requests.push(request)
    }
    pub fn add_data_source(&mut self, data_source: CsvDataSetConfig) {
        self.data_sources.push(data_source)
    }
    pub fn build(&self) -> Vec<u8> {
        let target: Vec<u8> = Vec::new();
        let mut writer = EventWriter::new(target);
        let script = root(&self);

        script.write(&mut writer);
        writer.export()
    }
    /// this function only used in example
    pub fn generate_file_bundle(&self, name: &str) {
        // 如果没有temp文件夹则创建temp文件夹
        let parent = "temp/".to_string();
        std::fs::create_dir_all(parent.clone()).unwrap_or_default();

        // 创建jmeter脚本文件
        let jmeter_file = name.to_string() + ".jmx";
        let mut file = std::fs::File::create(parent.clone() + &jmeter_file).unwrap();
        std::io::Write::write_all(&mut file, &self.build()).unwrap();

        // 创建空的report文件夹
        let report_folder = name.to_string() + "_report";
        std::fs::remove_dir_all(parent.clone() + &report_folder).unwrap_or_default();
        std::fs::create_dir_all(parent.clone() + &report_folder).unwrap_or_default();

        // 创建执行jmeter脚本的命令 output文件都指定放入report文件夹
        let command_file = name.to_string() + ".ps1";
        let log_file = report_folder.clone() + "/" + name + ".log";
        let result_file = report_folder.clone() + "/" + name + ".jtl";
        let dashboard_folder = report_folder.clone() + "/" + name;
        let mut file = std::fs::File::create(parent.clone() + &command_file).unwrap();
        let command = "jmeter -n".to_string()
            + " -t "
            + &jmeter_file
            + " -j "
            + &log_file
            + " -l "
            + &result_file
            + " -e -o "
            + &dashboard_folder;
        std::io::Write::write_all(&mut file, &command.as_bytes()).unwrap();
    }
}
