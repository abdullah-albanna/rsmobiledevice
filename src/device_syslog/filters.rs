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

pub enum LogAction {
    Continue,
    Log,
    Break,
}
impl LogFilter {
    // Method to apply filters to a log line
    pub fn apply(&self, logs_data: &LogsData) -> LogAction {
        match self {
            LogFilter::Match(pattern) => {
                if pattern.is_match(logs_data.message) {
                    return LogAction::Log;
                }
                LogAction::Continue
            }
            LogFilter::Trigger(_) => todo!(),
            LogFilter::Untrigger(pattern) => {
                if pattern.is_match(logs_data.message) {
                    return LogAction::Break;
                }
                LogAction::Log
            }
            LogFilter::Process(processes) => {
                let process = logs_data.process;
                for proc in processes {
                    if !process.contains(proc) {
                        return LogAction::Continue;
                    }
                }
                LogAction::Log
            }
            LogFilter::Exclude(exclude_processes) => {
                let process = logs_data.process;
                for exproc in exclude_processes {
                    if process.contains(exproc) {
                        return LogAction::Continue;
                    }
                }
                LogAction::Log
            }
            LogFilter::Quiet => {
                let process = logs_data.process;

                if QUITE.contains(&process) {
                    return LogAction::Continue;
                }
                LogAction::Log
            }
            LogFilter::KernelOnly => {
                let process = logs_data.process;

                if !process.contains("kernel") {
                    return LogAction::Continue;
                }
                LogAction::Log
            }
            LogFilter::NoKernel => {
                let process = logs_data.process;

                if process.contains("kernel") {
                    return LogAction::Continue;
                }
                LogAction::Log
            }
            LogFilter::Nothing => LogAction::Log,
        }
    }
}
