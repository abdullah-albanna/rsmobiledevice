use std::collections::HashSet;

use crate::device_syslog::{constants::QUITE, LogsData};

use regex::Regex;

// Enum for Log Filters
#[derive(Debug, Clone)]
pub enum LogFilter {
    Match(Regex),
    Trigger(Regex),
    Untrigger(Regex),
    Process(HashSet<String>),
    Exclude(HashSet<String>),
    Quiet,
    KernelOnly,
    NoKernel,
    Nothing,
}

/// Enum to filter part of the log
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
    // Method to apply filters to a log line
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
