use plist_plus::error::PlistError;
use rusty_libimobiledevice::error::LockdowndError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceInfoError {
    #[error("Lockdownd Error: {0}")]
    LockdowndError(#[from] LockdowndError),

    #[error("Plist Error: {0}")]
    PlistError(#[from] PlistError),

    #[error("Key not found")]
    KeyNotFound,
}
