use rusty_libimobiledevice::error::{IdeviceError, LockdowndError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IDeviceErrors {
    #[error("IDevice Error: {0}")]
    IDeviceError(#[from] IdeviceError),
    #[error("Lockdownd Error: {0}")]
    LockdowndError(#[from] LockdowndError),
}
