use rusty_libimobiledevice::error::{AfcError, IdeviceError, LockdowndError};
use thiserror::Error;

pub use crate::{
    device_diagnostic::errors::DeviceDiagnosticError, device_info::errors::DeviceInfoError,
    device_installer::errors::DeviceInstallerError, device_syslog::errors::DeviceSysLogError,
};

pub trait DeviceNotFoundErrorTrait {
    fn device_not_found() -> Self;
}

pub trait LockdowndErrorTrait {
    fn lockdownd_error(error: LockdowndError) -> Self;
}

pub trait AFCClientErrorTrait {
    fn afcclient_error(error: AfcError) -> Self;
}

#[derive(Debug, Error)]
pub enum DeviceClientError {
    #[error("IDevice Error: {0}")]
    IDeviceError(#[from] IdeviceError),

    #[error("Lockdownd Error: {0}")]
    LockdowndError(#[from] LockdowndError),

    #[error("Device not found, make sure it's plugged")]
    DeviceNotFound,

    #[error("AFC Client Error: {0}")]
    AFCClientError(#[from] AfcError),
}

impl LockdowndErrorTrait for DeviceClientError {
    fn lockdownd_error(error: LockdowndError) -> Self {
        Self::LockdowndError(error)
    }
}
impl DeviceNotFoundErrorTrait for DeviceClientError {
    fn device_not_found() -> Self {
        Self::DeviceNotFound
    }
}

impl AFCClientErrorTrait for DeviceClientError {
    fn afcclient_error(error: AfcError) -> Self {
        Self::AFCClientError(error)
    }
}
