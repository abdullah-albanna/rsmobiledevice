pub mod constants;
pub mod errors;
pub mod filters;
pub mod logs_data;

pub use errors::DeviceSysLogError;
pub use filters::{LogAction, LogFilter};
pub use logs_data::LogsData;

use crate::{device::DeviceClient, devices::SingleDevice};
use crossbeam_channel::{unbounded, Receiver, Sender};
use rusty_libimobiledevice::service::ServiceClient;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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

            let device = devices_clone.get_device().unwrap();
            let mut lockdown = devices_clone.get_lockdown_client().unwrap();
            let lockdown_service = lockdown
                .start_service("com.apple.syslog_relay", true)
                .unwrap();
            let service = ServiceClient::new(device, lockdown_service).unwrap();

            loop {
                // Listen for commands to start/stop logging

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

    pub fn log_to_stdout(&self) -> Result<(), DeviceSysLogError> {
        self.sender.send(LoggerCommand::StartLogging)?;
        self._start_service(|logs| println!("{}", logs.get_parsed_log_colored()));
        Ok(())
    }

    pub fn log_to_file<S>(&self, file_path: &S) -> Result<(), DeviceSysLogError>
    where
        S: AsRef<Path> + ?Sized + Sync,
    {
        self.sender.send(LoggerCommand::StartLogging)?;
        let file_path = file_path.as_ref().to_path_buf();

        self._start_service(move |logs| {
            // resolved path, just in case
            let resolved_path = fs::canonicalize(&file_path).unwrap_or_default();

            let mut file = match OpenOptions::new()
                .append(true)
                .create(true)
                .open(resolved_path)
            {
                Ok(file) => file,
                Err(_) => {
                    // Fallback to temp.log if the file cannot be opened or created
                    eprintln!("Failed to open log file, using default temp.log");
                    File::create("temp.log").unwrap()
                }
            };

            if let Err(e) = file.write_all(logs.get_parsed_log().as_bytes()) {
                eprintln!("Error writing to file: {}", e);
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
