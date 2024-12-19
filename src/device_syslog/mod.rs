pub mod constants;
pub mod errors;
pub mod filters;
pub mod logs_data;

pub use errors::DeviceSysLogError;
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
use std::thread;
use std::time::Duration;

const DEVICE_SYSLOG_SERVICE: &str = "com.apple.syslog_relay";

/// Enum for controlling logging behavior
#[derive(Debug, Clone)]
pub enum LoggerCommand {
    StartLogging,
    StopLogging,
}

#[derive(Debug)]
pub struct DeviceSysLog<T> {
    devices: Arc<DeviceClient<T>>,
    sender: Sender<LoggerCommand>,
    receiver: Arc<Receiver<LoggerCommand>>,
    filter: Arc<LogFilter>,
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
            _phantom: std::marker::PhantomData::<T>,
        }
    }
    pub fn new_from_arc(devices: Arc<DeviceClient<T>>) -> DeviceSysLog<T> {
        let (tx, rx) = unbounded();
        DeviceSysLog {
            devices,
            sender: tx,
            receiver: Arc::new(rx),
            filter: Arc::new(LogFilter::Nothing),
            _phantom: std::marker::PhantomData::<T>,
        }
    }
}

impl DeviceSysLog<SingleDevice> {
    /// Starts the logger service on a new thread
    fn _start_service<F>(&self, callback: F)
    where
        F: Fn(LogsData) + 'static + Sync + Send,
    {
        let devices_clone = Arc::clone(&self.devices);
        let receiver_clone = Arc::clone(&self.receiver);
        let filter_clone = Arc::clone(&self.filter);

        // Spawn a new thread to handle logging at the background
        thread::spawn(move || {
            let mut current_status: LoggerCommand = LoggerCommand::StopLogging;

            let device = devices_clone.get_device();
            let mut lockdownd = devices_clone
                .get_lockdownd_client::<DeviceSysLogError>()
                .expect("Could't get the device lockdown");
            let lockdownd_service = lockdownd
                .start_service(DEVICE_SYSLOG_SERVICE, true)
                .expect("Could't start the syslog service");
            let service = ServiceClient::new(device, lockdownd_service)
                .expect("Could't create a service client for syslog");

            loop {
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
                                match filter_clone.apply(&logs_data) {
                                    LogAction::Continue => continue,
                                    LogAction::Break => break,
                                    LogAction::Log => callback(logs_data),
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to receive data: {}", err);
                            thread::sleep(Duration::from_secs(1));
                        }
                    },
                    LoggerCommand::StopLogging => break,
                }
            }
        });
    }

    pub fn set_filter(&mut self, filter: LogFilter) {
        self.filter = filter.into();
    }

    pub fn log_to_custom<F>(&self, callback: F) -> Result<(), DeviceSysLogError>
    where
        F: Fn(LogsData) + 'static + Sync + Send,
    {
        self.devices.check_connected::<DeviceSysLogError>()?;
        self.sender.send(LoggerCommand::StartLogging)?;
        self._start_service(callback);
        Ok(())
    }
    pub fn log_to_stdout(&self) -> Result<(), DeviceSysLogError> {
        self.devices.check_connected::<DeviceSysLogError>()?;
        self.sender.send(LoggerCommand::StartLogging)?;
        self._start_service(|logs| println!("{}", logs.get_parsed_log_colored()));
        Ok(())
    }

    pub fn log_to_file<S>(&self, file_path: &S) -> Result<(), DeviceSysLogError>
    where
        S: AsRef<Path> + ?Sized + Sync,
    {
        self.devices.check_connected::<DeviceSysLogError>()?;
        self.sender.send(LoggerCommand::StartLogging)?;
        let file_path = file_path.as_ref().to_path_buf();

        self._start_service(move |logs| {
            // resolved path, just in case
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
        });
        Ok(())
    }

    pub fn stop_logging(&self) -> Result<(), DeviceSysLogError> {
        self.sender.send(LoggerCommand::StopLogging)?;
        Ok(())
    }
}
