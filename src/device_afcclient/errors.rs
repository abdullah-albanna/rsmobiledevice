use crate::errors::{AfcClientErrorTrait, DeviceNotFoundErrorTrait};
use rusty_libimobiledevice::error::AfcError;

use super::models::pathinfo::FileType;

#[derive(Debug, thiserror::Error)]
pub enum DeviceAfcClientError {
    #[error("Afc Error: {0}")]
    AfcError(#[from] AfcError),

    #[error("Device not found, make sure it's plugged")]
    DeviceNotFound,

    #[error("the path aready exists")]
    AlreadyExists,

    #[error("the provided directory is not empty")]
    DirectoryNotEmpty,

    #[error("the path `{0}` does not exists")]
    PathNotFound(String),

    #[error("invalid open option")]
    InvalidOpenOption,

    #[error("wrong kind of path, path: {path}\nexpected: {expected}\nfound: {found}")]
    WrongKind {
        path: String,
        expected: FileType,
        found: FileType,
    },
}

impl AfcClientErrorTrait for DeviceAfcClientError {
    fn afcclient_error(error: rusty_libimobiledevice::error::AfcError) -> Self {
        Self::AfcError(error)
    }
}

impl DeviceNotFoundErrorTrait for DeviceAfcClientError {
    fn device_not_found() -> Self {
        Self::DeviceNotFound
    }
}
