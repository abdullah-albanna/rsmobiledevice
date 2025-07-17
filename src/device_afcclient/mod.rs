use std::marker::PhantomData;
mod errors;
mod models;

use models::{pathinfo::FileType, FSTree, FileInfo};

use errors::DeviceAfcClientError;
use rusty_libimobiledevice::services::afc;

use crate::{device::DeviceClient, devices_collection::SingleDevice};

#[derive(Debug, Clone)]
pub struct DeviceAfcClient<'a, T> {
    device: &'a DeviceClient<T>,
    _p: PhantomData<T>,
}

impl<'a, T> DeviceAfcClient<'a, T> {
    pub fn new(device: &'a DeviceClient<T>) -> DeviceAfcClient<'a, T> {
        DeviceAfcClient {
            device,
            _p: PhantomData::<T>,
        }
    }
}

impl DeviceAfcClient<'_, SingleDevice> {
    pub fn list_directory(&self, path: &str) -> Result<Vec<String>, DeviceAfcClientError> {
        let afcclient = self
            .device
            .get_dynamic_afc_client::<DeviceAfcClientError>()?;

        let file_info = self.get_path_info(path)?;

        if matches!(file_info.st_ifmt, FileType::Directory) {
            Ok(afcclient.read_directory(path)?)
        } else {
            Err(DeviceAfcClientError::NonDir)
        }
    }

    pub fn create_directory(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        let afcclient = self
            .device
            .get_dynamic_afc_client::<DeviceAfcClientError>()?;

        if self.path_exists(path)? {
            return Err(DeviceAfcClientError::AlreadyExists);
        }

        Ok(afcclient.make_directory(path)?)
    }

    pub fn remove_file(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        let afcclient = self
            .device
            .get_dynamic_afc_client::<DeviceAfcClientError>()?;

        let file_info = self.get_path_info(path)?;

        if matches!(file_info.st_ifmt, FileType::File) {
            Ok(afcclient.remove_path(path)?)
        } else {
            Err(DeviceAfcClientError::NonFile)
        }
    }

    pub fn remove_directory(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        let afcclient = self
            .device
            .get_dynamic_afc_client::<DeviceAfcClientError>()?;

        if !self.path_exists(path)? {
            return Err(DeviceAfcClientError::PathNotFound(path.into()));
        }

        if !self.is_directory(path)? {
            return Err(DeviceAfcClientError::NonDir);
        }

        if !self.list_directory(path)?.is_empty() {
            return Err(DeviceAfcClientError::DirectoryNotEmpty);
        }

        Ok(afcclient.remove_path(path)?)
    }

    pub fn get_path_info(&self, path: &str) -> Result<FileInfo, DeviceAfcClientError> {
        let afcclient = self
            .device
            .get_dynamic_afc_client::<DeviceAfcClientError>()?;

        let mut file_info = afcclient.get_file_info(path)?;

        Ok(FileInfo::new(&mut file_info))
    }

    pub fn rename(&self, src: &str, dst: &str) -> Result<(), DeviceAfcClientError> {
        let afcclient = self
            .device
            .get_dynamic_afc_client::<DeviceAfcClientError>()?;

        Ok(afcclient.rename_path(src, dst)?)
    }

    pub fn path_exists(&self, path: &str) -> Result<bool, DeviceAfcClientError> {
        Ok(self.get_path_info(path).is_ok())
    }

    pub fn is_directory(&self, path: &str) -> Result<bool, DeviceAfcClientError> {
        let file_info = self.get_path_info(path)?;

        Ok(matches!(file_info.st_ifmt, FileType::Directory))
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, DeviceAfcClientError> {
        todo!()
    }

    pub fn write_file(&self, path: &str, contents: &[u8]) -> Result<(), DeviceAfcClientError> {
        todo!()
    }

    pub fn append_file(&self, path: &str, contents: &[u8]) -> Result<(), DeviceAfcClientError> {
        todo!()
    }

    pub fn copy_file_recursive(&self, src: &str, dst: &str) -> Result<(), DeviceAfcClientError> {
        todo!()
    }

    pub fn dump_fs_tree(&self, path: &str) -> Result<FSTree, DeviceAfcClientError> {
        todo!()
    }

    pub fn list_root(&self) -> Result<Vec<String>, DeviceAfcClientError> {
        todo!()
    }
}
