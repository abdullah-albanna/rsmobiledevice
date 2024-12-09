use crate::errors::{DeviceNotFoundErrorTrait, LockdowndErrorTrait};
use plist_plus::error::PlistError;
use rusty_libimobiledevice::error::LockdowndError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceInfoError {
    #[error("Plist Error: {0}")]
    PlistError(#[from] PlistError),

    #[error("Key not found")]
    KeyNotFound,

    #[error("Lockdownd Error: {0}")]
    LockdowndError(#[from] LockdowndError),

    #[error("Device not found, make sure it's plugged")]
    DeviceNotFound,
}

impl DeviceNotFoundErrorTrait for DeviceInfoError {
    fn device_not_found() -> Self {
        Self::DeviceNotFound
    }
}

impl LockdowndErrorTrait for DeviceInfoError {
    fn lockdown_error(error: LockdowndError) -> Self {
        Self::LockdowndError(error)
    }
}
