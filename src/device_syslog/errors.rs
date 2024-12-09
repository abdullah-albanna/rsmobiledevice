use crate::{device_syslog::LoggerCommand, errors::LockdowndErrorTrait};
use crossbeam_channel::SendError;
use rusty_libimobiledevice::error::LockdowndError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceSysLogError {
    #[error("Couldn't send a message to the channel, maybe it's closed?, error: {0}")]
    SendError(#[from] SendError<LoggerCommand>),

    #[error("Lockdownd Error: {0}")]
    LockdowndError(#[from] LockdowndError),
}

impl LockdowndErrorTrait for DeviceSysLogError {
    fn lockdown_error(error: rusty_libimobiledevice::error::LockdowndError) -> Self {
        Self::LockdowndError(error)
    }
}
