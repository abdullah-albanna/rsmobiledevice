use plist_plus::error::PlistError;
use rusty_libimobiledevice::error::{DiagnosticsRelayError, LockdowndError};
use thiserror::Error;

use crate::errors::DeviceClientError;

#[derive(Debug, Error)]
pub enum DeviceDiagnosticError {
    #[error("Diagnostics Relay Error: {0}")]
    DiagnosticsRelayError(#[from] DiagnosticsRelayError),

    #[error("Couldn't create a new diagnostics relay, error: {0}")]
    RelayInitializationError(String),

    #[error("Couldn't start the diagnostics service, error: {0}")]
    ServiceError(String),

    #[error("Device was not found")]
    DeviceNotFound,

    #[error("Lockdownd Error: {0}")]
    LockdowndError(#[from] LockdowndError),

    #[error("Device Client Error: {0}")]
    DeviceClientError(#[from] DeviceClientError),

    #[error("Plist error: {0}")]
    PlistError(#[from] PlistError),
}
