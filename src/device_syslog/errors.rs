use crate::{
    device_syslog::LoggerCommand,
    errors::{DeviceNotFoundErrorTrait, LockdowndErrorTrait},
};
use crossbeam_channel::SendError;
use rusty_libimobiledevice::error::LockdowndError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceSysLogError {
    #[error("Couldn't send a message to the channel, maybe it's closed?, error: {0}")]
    SendError(#[from] SendError<LoggerCommand>),

    #[error("Lockdownd Error: {0}")]
    LockdowndError(#[from] LockdowndError),

    #[error("Device not found, make sure it's plugged")]
    DeviceNotFound,
}

impl LockdowndErrorTrait for DeviceSysLogError {
    fn lockdownd_error(error: rusty_libimobiledevice::error::LockdowndError) -> Self {
        Self::LockdowndError(error)
    }
}

impl DeviceNotFoundErrorTrait for DeviceSysLogError {
    fn device_not_found() -> Self {
        Self::DeviceNotFound
    }
}
