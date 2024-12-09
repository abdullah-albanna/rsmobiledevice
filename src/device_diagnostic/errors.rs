use crate::errors::{DeviceNotFoundErrorTrait, LockdowndErrorTrait};
use plist_plus::error::PlistError;
use rusty_libimobiledevice::error::{DiagnosticsRelayError, LockdowndError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceDiagnosticError {
    #[error("Diagnostics Relay Error: {0}")]
    DiagnosticsRelayError(#[from] DiagnosticsRelayError),

    #[error("Couldn't create a new diagnostics relay, error: {0}")]
    RelayInitializationError(String),

    #[error("Couldn't start the diagnostics service, error: {0}")]
    ServiceError(String),

    #[error("Plist error: {0}")]
    PlistError(#[from] PlistError),

    #[error("Lockdownd Error: {0}")]
    LockdowndError(#[from] LockdowndError),

    #[error("Device not found, make sure it's plugged")]
    DeviceNotFound,
}

impl DeviceNotFoundErrorTrait for DeviceDiagnosticError {
    fn device_not_found() -> Self {
        Self::DeviceNotFound
    }
}

impl LockdowndErrorTrait for DeviceDiagnosticError {
    fn lockdown_error(error: LockdowndError) -> Self {
        Self::LockdowndError(error)
    }
}
