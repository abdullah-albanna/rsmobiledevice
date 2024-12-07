use crate::{device::DeviceClient, devices::SingleDevice};
use regex::Regex;
use rusty_libimobiledevice::service::ServiceClient;
use std::fmt;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Enum for controlling logging behavior
#[derive(Debug, Clone)]
enum LoggerCommand {
    StartLogging,
    StopLogging,
}

/// Struct to store parsed log data
#[derive(Debug, Default)]
struct LogsData<'a> {
    date: &'a str,
    device: &'a str,
    process: &'a str,
    pid: Option<&'a str>,
    severity: Option<&'a str>,
    message: &'a str,
}

/// Function to process a single log line
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

/// Function to process and display log data from a byte slice
fn process_logs(data: &[u8]) {
    let log_regex = Regex::new(r"^(?P<date>\w{3}\s+\d{1,2}\s+\d{2}:\d{2}:\d{2})\s+(?P<device>\S+)\s+(?P<process>[^\[\(<]+(?:\([^\)]+\))?)(?:\[(?P<pid>\d+)\])?\s*(?:<(?P<severity>\w+)>:\s*)?(?P<message>.+)$").unwrap();

    let logs_raw_string = String::from_utf8_lossy(data);

    for line in logs_raw_string.split_terminator('\n') {
        let line = line.trim_matches('\0'); // Remove null characters
        if let Some(logs_data) = process_log_line(line, &log_regex) {
            println!("{:#?}", logs_data);
        } else {
            eprintln!("Unparsed log line: {}", line);
        }
    }
}

/// A struct representing the logging service for a specific device
pub struct DeviceSysLog<T> {
    devices: Arc<DeviceClient<T>>,
    sender: mpsc::Sender<LoggerCommand>,
    receiver: Arc<Mutex<mpsc::Receiver<LoggerCommand>>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> DeviceSysLog<T> {
    pub fn new(devices: DeviceClient<T>) -> DeviceSysLog<T> {
        let (tx, rx) = mpsc::channel();
        DeviceSysLog {
            devices: Arc::new(devices),
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
            _phantom: std::marker::PhantomData::<T>,
        }
    }
}

impl DeviceSysLog<SingleDevice> {
    /// Starts the logger service on a new thread
    fn _start_service(&self) {
        let devices_clone = Arc::clone(&self.devices);
        let receiver_clone = Arc::clone(&self.receiver);

        // Spawn a new thread to handle logging
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

                let reciver = receiver_clone.lock();

                if let Err(err) = reciver {
                    eprint!("Error: {:?}", err);
                    continue;
                }
                let reciver = reciver.unwrap();

                if let Ok(command) = reciver.try_recv() {
                    current_status = command;
                }

                match current_status {
                    LoggerCommand::StartLogging => match service.receive(1024) {
                        Ok(data) => {
                            process_logs(&data);
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

    /// Request to start logging
    pub fn start_logging(&self) {
        self.sender.send(LoggerCommand::StartLogging).unwrap();
        self._start_service();
    }

    /// Request to stop logging
    pub fn stop_logging(&self) {
        self.sender.send(LoggerCommand::StopLogging).unwrap();
    }
}

impl fmt::Debug for DeviceSysLog<SingleDevice> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeviceSysLog")
            .field("devices", &self.devices)
            .finish()
    }
}
