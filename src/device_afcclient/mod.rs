use std::marker::PhantomData;
mod errors;
mod models;

use models::FSTree;

use errors::DeviceAfcClientError;

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
    pub fn read_directory(&self, path: &str) -> Result<Vec<String>, DeviceAfcClientError> {
        todo!()
    }
    pub fn create_directory(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        todo!()
    }
    pub fn remove_file(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        todo!()
    }
    pub fn remove_directory(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        todo!()
    }
    pub fn rename(&self, src: &str, dst: &str) -> Result<(), DeviceAfcClientError> {
        todo!()
    }
    pub fn file_exists(&self, path: &str) -> Result<bool, DeviceAfcClientError> {
        todo!()
    }
    pub fn is_directory(&self, path: &str) -> Result<bool, DeviceAfcClientError> {
        todo!()
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
