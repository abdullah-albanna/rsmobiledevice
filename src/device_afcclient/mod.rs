file_info.st_ifmee std::marker::PhantomData;
mod errors;
mod models;

use models::{
    afc_open_options::AfcOpenOptions, device_file::DeviceFile, pathinfo::FileType, FSTree, FileInfo,
};

use errors::DeviceAfcClientError;
use rusty_libimobiledevice::{
    idevice::Device,
    services::afc::{self, AfcClient, AfcFileMode},
};
use zip::unstable::write;

use crate::{device::DeviceClient, devices_collection::SingleDevice, errors::DeviceClientError};

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
    fn afc(&self) -> Result<AfcClient<'_>, DeviceAfcClientError> {
        self.device.get_dynamic_afc_client::<DeviceAfcClientError>()
    }
    pub fn list_directory(&self, path: &str) -> Result<Vec<String>, DeviceAfcClientError> {
        let file_info = self.get_path_info(path)?;

        if file_info.is_dir() {
            Ok(self.afc()?.read_directory(path)?)
        } else {
            Err(DeviceAfcClientError::WrongKind {
                path: path.into(),
                expected: FileType::Directory,
                found: file_info.st_ifmt,
            })
        }
    }

    pub fn create_directory(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        if self.path_exists(path)? {
            return Err(DeviceAfcClientError::AlreadyExists);
        }

        Ok(self.afc()?.make_directory(path)?)
    }

    pub fn remove_file(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        let file_info = self.get_path_info(path)?;
        if file_info.is_file() {
            Ok(self.afc()?.remove_path(path)?)
        } else {
            Err(DeviceAfcClientError::WrongKind {
                path: path.into(),
                expected: FileType::File,
                found: file_info.st_ifmt,
            })
        }
    }

    pub fn remove_directory(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        if !self.path_exists(path)? {
            return Err(DeviceAfcClientError::PathNotFound(path.into()));
        }

        let file_info = self.get_path_info(path)?;
        if !self.is_directory(path)? {
            return Err(DeviceAfcClientError::WrongKind {
                path: path.into(),
                expected: FileType::Directory,
                found: file_info.st_ifmt,
            });
        }

        if !self.list_directory(path)?.is_empty() {
            return Err(DeviceAfcClientError::DirectoryNotEmpty);
        }

        Ok(self.afc()?.remove_path(path)?)
    }

    pub fn get_path_info(&self, path: &str) -> Result<FileInfo, DeviceAfcClientError> {
        let mut file_info = self.afc()?.get_file_info(path)?;

        Ok(FileInfo::new(&mut file_info))
    }

    pub fn rename(&self, src: &str, dst: &str) -> Result<(), DeviceAfcClientError> {
        Ok(self.afc()?.rename_path(src, dst)?)
    }

    pub fn path_exists(&self, path: &str) -> Result<bool, DeviceAfcClientError> {
        Ok(self.get_path_info(path).is_ok())
    }

    pub fn is_directory(&self, path: &str) -> Result<bool, DeviceAfcClientError> {
        if !self.path_exists(path)? {
            return Err(DeviceAfcClientError::PathNotFound(path.into()));
        }

        Ok(self.get_path_info(path)?.is_dir())
    }

    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, DeviceAfcClientError> {
        if !self.path_exists(path)? {
            return Err(DeviceAfcClientError::PathNotFound(path.into()));
        }

        let file_info = self.get_path_info(path)?;
        if !file_info.is_file() {
            return Err(DeviceAfcClientError::WrongKind {
                path: path.into(),
                expected: FileType::File,
                found: file_info.st_ifmt,
            });
        }

        let afc = self.afc()?;

        let handle = afc.file_open(path, AfcFileMode::ReadOnly)?;
        let content = afc.file_read(handle, file_info.st_size as _)?;

        afc.file_close(handle)?;

        Ok(content)
    }

    pub fn write_file(
        &self,
        path: &str,
        contents: impl Into<Vec<u8>>,
    ) -> Result<(), DeviceAfcClientError> {
        if self.path_exists(path)? {
            return Err(DeviceAfcClientError::AlreadyExists);
        }

        let afc = self.afc()?;

        let handle = afc.file_open(path, AfcFileMode::WriteOnly)?;

        afc.file_write(handle, contents.into())?;
        Ok(afc.file_close(handle)?)
    }

    pub fn append_file(
        &self,
        path: &str,
        contents: impl Into<Vec<u8>>,
    ) -> Result<(), DeviceAfcClientError> {
        if !self.path_exists(path)? {
            return Err(DeviceAfcClientError::PathNotFound(path.into()));
        }

        let file_info = self.get_path_info(path)?;
        if !matches!(file_info.st_ifmt, FileType::File) {
            return Err(DeviceAfcClientError::WrongKind {
                path: path.into(),
                expected: FileType::File,
                found: file_info.st_ifmt,
            });
        }

        let afc = self.afc()?;

        let handle = afc.file_open(path, AfcFileMode::Append)?;
        afc.file_write(handle, contents.into())?;
        Ok(afc.file_close(handle)?)
    }

    pub fn truncate_file(&self, path: &str) -> Result<(), DeviceAfcClientError> {
        if !self.path_exists(path)? {
            return Err(DeviceAfcClientError::PathNotFound(path.into()));
        }

        let file_info = self.get_path_info(path)?;
        if !matches!(file_info.st_ifmt, FileType::File) {
            return Err(DeviceAfcClientError::WrongKind {
                path: path.into(),
                expected: FileType::File,
                found: file_info.st_ifmt,
            });
        }

        let afc = self.afc()?;

        let handle = afc.file_open(path, AfcFileMode::WriteOnly)?;
        afc.file_truncate(handle, 0)?;
        Ok(afc.file_close(handle)?)
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

impl<'a> DeviceAfcClient<'a, SingleDevice> {
    pub fn open(
        &'a self,
        path: &str,
        options: AfcOpenOptions,
    ) -> Result<DeviceFile<'a>, DeviceAfcClientError> {
        let afc = self.afc()?;
        let exists = self.path_exists(path)?;

        match (exists, options.create) {
            (false, false) => {
                return Err(DeviceAfcClientError::PathNotFound(path.into()));
            }
            (false, true) => {}
            (true, true) => {
                return Err(DeviceAfcClientError::AlreadyExists);
            }
            _ => {}
        }

        let mode = match (options.append, options.read, options.write) {
            (true, true, false | true) => AfcFileMode::ReadAppend,
            (false, true, true) => AfcFileMode::ReadWrite,
            (false, true, false) => AfcFileMode::ReadOnly,
            (false, false, true) => AfcFileMode::WriteOnly,
            (true, false, false | true) => AfcFileMode::Append,
            (false, false, false) => return Err(DeviceAfcClientError::InvalidOpenOption),
        };

        if exists && options.truncate {
            self.truncate_file(path)?;
        }

        let fd = afc.file_open(path, mode)?;

        Ok(DeviceFile { afc, handle: fd })
    }
}
