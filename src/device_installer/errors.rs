use plist_plus::error::PlistError;
use rusty_libimobiledevice::error::{AfcError, InstProxyError};
use thiserror::Error;

use crate::errors::{AfcClientErrorTrait, DeviceNotFoundErrorTrait};

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

impl AfcClientErrorTrait for DeviceInstallerError {
    fn afcclient_error(error: AfcError) -> Self {
        Self::AfcClientError(error)
    }
}

impl DeviceNotFoundErrorTrait for DeviceInstallerError {
    fn device_not_found() -> Self {
        Self::DeviceNotFound
    }
}
