use regex::{Captures, Regex};

/// Struct to store parsed log data
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LogsData<'a> {
    pub(crate) date: &'a str,
    pub(crate) device: &'a str,
    pub(crate) process: &'a str,
    pub(crate) pid: Option<&'a str>,
    pub(crate) severity: Option<&'a str>,
    pub(crate) message: &'a str,
}

impl<'a> LogsData<'a> {
    pub(crate) fn get_parsed_log(&self) -> String {
        format!(
            "[{}] {} {} [{}] <{}>: {}",
            self.date,
            self.device,
            self.process,
            self.pid.map_or("None", |d: &str| d),
            self.severity.map_or("None", |d: &str| d),
            self.message
        )
    }

    pub(crate) fn get_parsed_log_colored(&self) -> String {
        format!(
            "[\x1b[34m{}\x1b[0m] \x1b[32m{}\x1b[0m \x1b[36m{}\x1b[0m [{}] <\x1b[31m{}\x1b[0m>: \x1b[37m{}\x1b[0m",
            self.date,
            self.device,
            self.process,
            self.pid.map_or("None", |d: &str| d),
            self.severity.map_or("None", |d: &str| d),
            self.message
        )
    }
}

fn process_log_line<'a>(line: &'a str, log_regex: &Regex) -> Option<LogsData<'a>> {
    // a helper to get the captures value or default
    fn get_capture<'b>(captures: &Captures<'b>, name: &str, default: &'b str) -> &'b str {
        captures.name(name).map_or(default, |m| m.as_str())
    }

    log_regex.captures(line).map(|captures| LogsData {
        date: get_capture(&captures, "date", "unknown"),
        device: get_capture(&captures, "device", "unknown"),
        process: get_capture(&captures, "process", "unknown"),
        pid: captures.name("pid").map(|m| m.as_str()), // Optional field
        severity: captures.name("severity").map(|m| m.as_str()), // Optional field
        message: get_capture(&captures, "message", "unknown"),
    })
}

impl<'a> From<&'a str> for LogsData<'a> {
    fn from(value: &'a str) -> Self {
        let log_regex = Regex::new(r"^(?P<date>\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+(?P<device>\S+)\s+(?P<process>[^\[\(<]+(?:\([^\)]+\))?)(?:\[(?P<pid>\d+)\])?\s*(?:<(?P<severity>\w+)>:\s*)?(?P<message>.+)$").expect("Couldn't create a new regex");

        process_log_line(value, &log_regex).map_or(LogsData::default(), |l| l)
    }
}
