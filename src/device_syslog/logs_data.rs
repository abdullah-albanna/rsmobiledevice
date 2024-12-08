use regex::Regex;

/// Struct to store parsed log data
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LogsData<'a> {
    date: &'a str,
    device: &'a str,
    pub(crate) process: &'a str,
    pid: Option<&'a str>,
    severity: Option<&'a str>,
    pub(crate) message: &'a str,
}

impl<'a> LogsData<'a> {
    pub(crate) fn get_parsed_log(&self) -> String {
        format!(
            "[{}] {} {} [{}] <{}>: {}",
            self.date,
            self.device,
            self.process,
            self.pid.unwrap_or("None"),
            self.severity.unwrap_or("None"),
            self.message
        )
    }

    pub(crate) fn get_parsed_log_colored(&self) -> String {
        format!(
        "[\x1b[34m{}\x1b[0m] \x1b[32m{}\x1b[0m \x1b[36m{}\x1b[0m [{}] <\x1b[31m{}\x1b[0m>: \x1b[37m{}\x1b[0m",
        self.date,
        self.device,
        self.process,
        self.pid.unwrap_or("None"),
        self.severity.unwrap_or("None"),
        self.message
    )
    }
}

fn process_log_line<'a>(line: &'a str, log_regex: &Regex) -> Option<LogsData<'a>> {
    log_regex.captures(line).map(|captures| LogsData {
        date: captures.name("date").unwrap().as_str(),
        device: captures.name("device").unwrap().as_str(),
        process: captures.name("process").unwrap().as_str(),
        pid: captures.name("pid").map(|m| m.as_str()), // Optional
        severity: captures.name("severity").map(|m| m.as_str()), // Optional
        message: captures.name("message").unwrap().as_str(),
    })
}

impl<'a> From<&'a str> for LogsData<'a> {
    fn from(value: &'a str) -> Self {
        let log_regex = Regex::new(r"^(?P<date>\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+(?P<device>\S+)\s+(?P<process>[^\[\(<]+(?:\([^\)]+\))?)(?:\[(?P<pid>\d+)\])?\s*(?:<(?P<severity>\w+)>:\s*)?(?P<message>.+)$").unwrap();

        process_log_line(value, &log_regex).unwrap_or_default()
    }
}
