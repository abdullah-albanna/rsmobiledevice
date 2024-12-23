use crate::device_syslog::{constants::QUITE, LogsData};
use regex::Regex;
use std::collections::HashSet;

/// Enum representing different types of log filters.
///
/// This enum allows the application of various filters to log entries, such as matching patterns, excluding certain processes, or triggering actions based on specific conditions.
///
/// The filters can be categorized into:
/// - **Match**: Matches a specific regular expression.
/// - **Trigger**: Used for triggering actions on a log, but this is not yet implemented.
/// - **Untrigger**: Matches a regular expression, if it was found, it stop the logging
/// - **Process**: Filters logs based on the process name.
/// - **Exclude**: Filters logs by excluding certain processes.
/// - **OneShot**: Doesn't log anything up until it finds the pattern then it stops
/// - **Quiet**: Filters out noisy process defined by `libimobiledevice` list.
/// - **KernelOnly**: Used to only log the kernel.
/// - **NoKernel**: Used to log everything but kernel
/// - **Nothing**: This filter performs no operation (acts as a no-op).
#[derive(Debug, Clone)]
pub enum LogFilter {
    Match(Regex),
    Trigger(Regex),
    Untrigger(Regex),
    Process(HashSet<String>),
    Exclude(HashSet<String>),
    OneShot(Regex),
    Quiet,
    KernelOnly,
    NoKernel,
    Nothing,
}

/// Enum representing different parts of a log entry that can be filtered.
///
/// This enum is used to specify which part of a log line should be considered when applying the filter:
/// - **Date**: The date of the log entry.
/// - **Device**: The device that generated the log entry.
/// - **Process**: The process generating the log entry.
/// - **Pid**: The process ID associated with the log entry.
/// - **Severity**: The severity level of the log entry.
/// - **Message**: The message content of the log entry.
/// - **All**: Applies the filter to all parts of the log entry.
#[derive(Debug, Clone)]
pub enum FilterPart {
    Date,
    Device,
    Process,
    Pid,
    Severity,
    Message,
    All,
}

pub enum LogAction {
    Continue,
    Log,
    Break,
}

impl LogFilter {
    /// Applies the filter to the given log entry based on the specified part of the log.
    ///
    /// This method checks which part of the log entry (date, device, process, pid, severity, message) should be filtered
    /// and applies the corresponding filter action (`Log`, `Continue`, or `Break`).
    ///
    /// # Arguments
    ///
    /// - `logs_data`: The `LogsData` struct containing the parsed log information.
    /// - `filter_part`: Specifies which part of the log entry to apply the filter to (e.g., Date, Device, Process, etc.).
    ///
    /// # Returns
    ///
    /// This function returns a `LogAction` that specifies the outcome:
    /// - `LogAction::Log` if the log passes the filter.
    /// - `LogAction::Continue` if the log is ignored.
    /// - `LogAction::Break` if it must stop the logging
    pub fn apply(&self, logs_data: &LogsData, filter_part: &FilterPart) -> LogAction {
        match filter_part {
            FilterPart::All => {
                return apply_match_on_part(
                    self,
                    &[
                        Some(logs_data.message),
                        logs_data.severity,
                        logs_data.pid,
                        Some(logs_data.device),
                        Some(logs_data.date),
                        Some(logs_data.process),
                    ],
                )
            }
            FilterPart::Pid => return apply_match_on_part(self, &[logs_data.pid]),
            FilterPart::Date => return apply_match_on_part(self, &[Some(logs_data.date)]),
            FilterPart::Device => return apply_match_on_part(self, &[Some(logs_data.device)]),
            FilterPart::Process => return apply_match_on_part(self, &[Some(logs_data.process)]),
            FilterPart::Message => return apply_match_on_part(self, &[Some(logs_data.message)]),
            FilterPart::Severity => return apply_match_on_part(self, &[logs_data.severity]),
        }

        /// Applies the specified `LogFilter` to a given set of log parts.
        ///
        /// This function matches the filter against the parts of the log, which may include message, severity, PID, device, date, and process.
        /// Depending on the filter type, it either logs the action, continues, or breaks.
        ///
        /// # Arguments
        ///
        /// - `filter`: The `LogFilter` to apply.
        /// - `parts`: An array of optional string slices representing different parts of the log to apply on.
        ///
        /// # Returns
        ///
        /// Returns a `LogAction` based on the filter's action.
        fn apply_match_on_part(filter: &LogFilter, parts: &[Option<&str>]) -> LogAction {
            match filter {
                LogFilter::Match(pattern) => {
                    for part in parts.iter().flatten() {
                        if pattern.is_match(part) {
                            return LogAction::Log;
                        }
                    }
                    LogAction::Continue
                }
                LogFilter::Trigger(_) => todo!(),
                LogFilter::Untrigger(pattern) => {
                    for part in parts.iter().flatten() {
                        if pattern.is_match(part) {
                            return LogAction::Break;
                        }
                    }
                    LogAction::Log
                }
                LogFilter::Process(processes) => {
                    for part in parts.iter().flatten() {
                        for proc in processes {
                            if !part.contains(proc) {
                                return LogAction::Continue;
                            }
                        }
                    }
                    LogAction::Log
                }
                LogFilter::Exclude(exclude_processes) => {
                    for part in parts.iter().flatten() {
                        for exproc in exclude_processes {
                            if part.contains(exproc) {
                                return LogAction::Continue;
                            }
                        }
                    }
                    LogAction::Log
                }
                LogFilter::OneShot(pattern) => {
                    for part in parts.iter().flatten() {
                        if pattern.is_match(part) {
                            return LogAction::Break;
                        }
                    }
                    LogAction::Continue
                }
                LogFilter::Quiet => {
                    for part in parts.iter().flatten() {
                        if QUITE.contains(part) {
                            return LogAction::Continue;
                        }
                    }
                    LogAction::Log
                }
                LogFilter::KernelOnly => {
                    for part in parts.iter().flatten() {
                        if !part.contains("kernel") {
                            return LogAction::Continue;
                        }
                    }
                    LogAction::Log
                }
                LogFilter::NoKernel => {
                    for part in parts.iter().flatten() {
                        if part.contains("kernel") {
                            return LogAction::Continue;
                        }
                    }
                    LogAction::Log
                }
                LogFilter::Nothing => LogAction::Log,
            }
        }
    }
}
