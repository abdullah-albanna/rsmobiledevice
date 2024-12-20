use regex::{Captures, Regex};

/// Struct to store parsed log data
///
/// This struct contains the parsed information of a log entry, including the date, device, process, optional
/// process ID, optional severity, and the log message.
///
/// Some are Optional because they may or may not exist in some logs
#[derive(Debug, Default, Clone, PartialEq)]
pub struct LogsData<'a> {
    /// The date the log entry was created (e.g., "Dec 20 14:22:15")
    pub date: &'a str,

    /// The device name or identifier that generated the log entry
    pub device: &'a str,

    /// The process name that generated the log entry
    pub process: &'a str,

    /// The process ID associated with the log entry, if available
    pub pid: Option<&'a str>,

    /// The severity level of the log entry (e.g., "error", "info"), if available
    pub severity: Option<&'a str>,

    /// The actual log message
    pub message: &'a str,
}

impl<'a> LogsData<'a> {
    /// Returns a formatted string representation of the log entry
    ///
    /// This method formats the log data into a structured string like:
    /// `[date] device process [pid] <severity>: message`.
    ///
    /// If `pid` or `severity` are `None`, they are replaced with the string `"None"`.
    ///
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

    /// Returns a colored, formatted string representation of the log entry
    ///
    /// This method formats the log data into a structured string with ANSI color codes for terminal output:
    /// - Date in blue
    /// - Device in green
    /// - Process in cyan
    /// - PID in red (if available)
    /// - Severity in red (if available)
    /// - Message in white
    ///
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

/// Helper function to process a log line and extract structured data
///
/// This function attempts to match a log line against a regular expression and extract fields like `date`,
/// `device`, `process`, `pid`, `severity`, and `message`. If the line matches the regex, it returns a `LogsData`
/// instance; otherwise, it returns `None`.
///
/// # Arguments
///
/// - `line`: The raw log line to be processed.
/// - `log_regex`: The regular expression used to parse the log line.
///
/// # Returns
///
/// This function returns an `Option<LogsData>`. If the line matches the regex, a `LogsData` instance is returned.
/// Otherwise, `None` is returned.
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
    /// Converts a raw log line into a `LogsData` instance
    ///
    /// This function uses a regular expression to parse the given log line and extract the relevant data.
    /// If the log line matches the expected format, a `LogsData` instance is created; otherwise, the default
    /// `LogsData` instance is returned.
    ///
    /// # Arguments
    ///
    /// - `value`: The raw log line as a string slice.
    ///
    /// # Returns
    ///
    /// A `LogsData` instance containing the parsed log information if found, or a default values.
    fn from(value: &'a str) -> Self {
        let log_regex = Regex::new(r"^(?P<date>\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+(?P<device>\S+)\s+(?P<process>[^\[\(<]+(?:\([^\)]+\))?)(?:\[(?P<pid>\d+)\])?\s*(?:<(?P<severity>\w+)>:\s*)?(?P<message>.+)$").expect("Couldn't create a new regex");

        process_log_line(value, &log_regex).unwrap_or_default()
    }
}
