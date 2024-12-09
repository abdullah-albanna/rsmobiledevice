use plist_plus::error::PlistError;
use rusty_libimobiledevice::error::{AfcError, IdeviceError, InstProxyError, LockdowndError};
use thiserror::Error;

pub use crate::device_diagnostic::errors::DeviceDiagnosticError;
pub use crate::device_info::errors::DeviceInfoError;
pub use crate::device_syslog::errors::DeviceSysLogError;

pub trait DeviceNotFoundErrorTrait {
    fn device_not_found() -> Self;
}

pub trait LockdowndErrorTrait {
    fn lockdown_error(error: LockdowndError) -> Self;
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
    fn lockdown_error(error: LockdowndError) -> Self {
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

#[derive(Debug, Error)]
pub enum DeviceInstallerError {
    #[error("Couldn't create a folder")]
    ErrorCreatingFolder,

    #[error("IO Error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Zip Error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("AFC Client Error: {0}")]
    AfcClientError(#[from] AfcError),

    #[error("The given package is neithr an ipa or an ipcc")]
    UnknownPackage,

    #[error("Plist Error: {0}")]
    PlistError(#[from] PlistError),

    #[error("Installation Proxy Error: {0}")]
    InstallationProxyError(#[from] InstProxyError),

    #[error("Device not found, make sure it's plugged")]
    DeviceNotFound,
}

impl AFCClientErrorTrait for DeviceInstallerError {
    fn afcclient_error(error: AfcError) -> Self {
        Self::AfcClientError(error)
    }
}

impl DeviceNotFoundErrorTrait for DeviceInstallerError {
    fn device_not_found() -> Self {
        Self::DeviceNotFound
    }
}
