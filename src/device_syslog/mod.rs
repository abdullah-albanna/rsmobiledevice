//! This module provides logging capabilities for iOS devices using the syslog service.
//! It includes options for logging to the console, custom callbacks, or files, with
//! filtering support for more controlled logging.
//!
//! ## Features
//! - Logs in the background using threads
//! - Start and stop logging from devices.
//! - Filter logs based on specific criteria.
//! - Output logs to custom destinations (stdout, files, or user-defined callbacks).

pub mod constants;
pub mod errors;
pub mod filters;
pub mod logs_data;

pub use errors::DeviceSysLogError;
use filters::FilterPart;
pub use filters::{LogAction, LogFilter};
pub use logs_data::LogsData;

use crate::{device::DeviceClient, devices_collection::SingleDevice};
use crossbeam_channel::{unbounded, Receiver, Sender};
use rusty_libimobiledevice::service::ServiceClient;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use std::time::Duration;

const DEVICE_SYSLOG_SERVICE: &str = "com.apple.syslog_relay";

/// Enum for controlling logging behavior.
///
/// This enum defines commands to start or stop the logging process.
#[derive(Debug, Clone)]
pub enum LoggerCommand {
    StartLogging,
    StopLogging,
}

/// Struct for managing syslog data from a device or a group of devices.
///
/// `DeviceSysLog` is a high-level interface for interacting with the syslog service of iOS devices.
///
/// # Type Parameters
/// - `T`: Determines whether the logger operates on a single device or a group of devices.
#[derive(Debug)]
pub struct DeviceSysLog<T> {
    devices: Arc<DeviceClient<T>>,
    sender: Sender<LoggerCommand>,
    receiver: Arc<Receiver<LoggerCommand>>,
    filter: Arc<LogFilter>,
    filter_part: Arc<FilterPart>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> DeviceSysLog<T> {
    pub fn new(devices: DeviceClient<T>) -> DeviceSysLog<T> {
        let (tx, rx) = unbounded();
        DeviceSysLog {
            devices: Arc::new(devices),
            sender: tx,
            receiver: Arc::new(rx),
            filter: Arc::new(LogFilter::Nothing),
            filter_part: Arc::new(FilterPart::All),
            _phantom: std::marker::PhantomData::<T>,
        }
    }

    /// Creates a new `DeviceSysLog` instance from an `Arc` of `DeviceClient`.
    ///
    /// This is useful when creating multiple DeviceSysLog from a single client
    pub fn new_from_arc(devices: Arc<DeviceClient<T>>) -> DeviceSysLog<T> {
        let (tx, rx) = unbounded();
        DeviceSysLog {
            devices,
            sender: tx,
            receiver: Arc::new(rx),
            filter: Arc::new(LogFilter::Nothing),
            filter_part: Arc::new(FilterPart::All),
            _phantom: std::marker::PhantomData::<T>,
        }
    }
}

impl DeviceSysLog<SingleDevice> {
    /// Internal method to start the logging service on a separate thread.
    ///
    /// # Parameters
    /// - `callback`: A function to handle the `LogsData` objects received from the device.
    ///
    /// # Returns
    /// A `JoinHandle` for the logging thread.
    fn _start_service<F>(&self, callback: F) -> JoinHandle<()>
    where
        F: Fn(LogsData) + 'static + Sync + Send,
    {
        let devices_clone = Arc::clone(&self.devices);
        let receiver_clone = Arc::clone(&self.receiver);
        let filter_clone = Arc::clone(&self.filter);
        let filter_part = Arc::clone(&self.filter_part);

        thread::spawn(move || {
            let mut current_status: LoggerCommand = LoggerCommand::StopLogging;

            let device = devices_clone.get_device();
            let mut lockdownd = devices_clone
                .get_lockdownd_client::<DeviceSysLogError>()
                .expect("Couldn't get the device lockdown client");
            let lockdownd_service = lockdownd
                .start_service(DEVICE_SYSLOG_SERVICE, true)
                .expect("Couldn't start the syslog service");
            let service = ServiceClient::new(device, lockdownd_service)
                .expect("Couldn't create a syslog service client");

            'log: loop {
                if let Ok(command) = receiver_clone.try_recv() {
                    current_status = command;
                }

                match current_status {
                    LoggerCommand::StartLogging => match service.receive(1024) {
                        Ok(data) => {
                            let logs_raw_string = String::from_utf8_lossy(&data);

                            for line in logs_raw_string.split_terminator('\n') {
                                let line = line.trim_matches('\0'); // Remove null characters

                                let logs_data = LogsData::from(line);
                                match filter_clone.apply(&logs_data, &filter_part) {
                                    LogAction::Continue => continue 'log,
                                    LogAction::Break => break 'log,
                                    LogAction::Log => callback(logs_data),
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to receive data: {}", err);
                            thread::sleep(Duration::from_secs(1));
                        }
                    },
                    LoggerCommand::StopLogging => break 'log,
                }
            }
        })
    }

    /// Sets the log filter for this `DeviceSysLog` instance.
    ///
    /// # Parameters
    /// - `filter`: The filter logic to apply to logs.
    /// - `filter_part`: Specifies which parts of the log to apply the filter on.
    pub fn set_filter(&mut self, filter: LogFilter, filter_part: FilterPart) {
        self.filter = filter.into();
        self.filter_part = filter_part.into();
    }

    /// Logs to a custom destination using the provided callback function.
    ///
    /// This is a non blocking function
    ///
    /// # Parameters
    /// - `callback`: A function to process the `LogsData`.
    pub fn log_to_custom<F>(&self, callback: F) -> Result<JoinHandle<()>, DeviceSysLogError>
    where
        F: Fn(LogsData) + 'static + Sync + Send,
    {
        self.devices.check_connected::<DeviceSysLogError>()?;
        self.sender.send(LoggerCommand::StartLogging)?;
        Ok(self._start_service(callback))
    }

    /// Logs to the console (stdout).
    ///
    /// This is a non blocking function
    pub fn log_to_stdout(&self) -> Result<JoinHandle<()>, DeviceSysLogError> {
        self.devices.check_connected::<DeviceSysLogError>()?;
        self.sender.send(LoggerCommand::StartLogging)?;
        Ok(self._start_service(|logs| println!("{}", logs.get_parsed_log_colored())))
    }

    /// Logs to a specified file.
    ///
    /// This is a non blocking function
    ///
    /// # Parameters
    /// - `file_path`: Path to the file where logs should be saved.
    pub fn log_to_file<S>(&self, file_path: &S) -> Result<JoinHandle<()>, DeviceSysLogError>
    where
        S: AsRef<Path> + ?Sized + Sync,
    {
        self.devices.check_connected::<DeviceSysLogError>()?;
        self.sender.send(LoggerCommand::StartLogging)?;
        let file_path = file_path.as_ref().to_path_buf();

        Ok(self._start_service(move |logs| {
            let resolved_path = match fs::canonicalize(&file_path) {
                Ok(path) => path,
                Err(_) => {
                    eprintln!("Failed to resolve file path: {}", file_path.display());
                    file_path.to_owned()
                }
            };
            let mut file = match OpenOptions::new()
                .append(true)
                .create(true)
                .open(&resolved_path)
            {
                Ok(file) => file,
                Err(e) => {
                    eprintln!(
                        "Critical error: Failed to open log file at {:?}: {}",
                        resolved_path, e
                    );
                    return;
                }
            };

            if let Err(e) = file.write_all(logs.get_parsed_log().as_bytes()) {
                eprintln!("Error writing to file: {}", e);
                return;
            }

            if let Err(e) = file.flush() {
                eprintln!("Error flushing to file: {}", e);
            }
        }))
    }

    pub fn stop_logging(&self) -> Result<(), DeviceSysLogError> {
        self.sender.send(LoggerCommand::StopLogging)?;
        Ok(())
    }
}
