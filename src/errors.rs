use plist_plus::error::PlistError;
use rusty_libimobiledevice::error::{IdeviceError, LockdowndError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IDeviceErrors {
    #[error("IDevice Error: {0}")]
    IDeviceError(#[from] IdeviceError),

    #[error("Lockdownd Error: {0}")]
    LockdowndError(#[from] LockdowndError),

    #[error("Plist Error: {0}")]
    PlistError(#[from] PlistError),

    #[error("Key not found")]
    KeyNotFound,
}
