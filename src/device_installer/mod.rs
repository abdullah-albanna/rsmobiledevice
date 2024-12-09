use std::{
    collections::HashMap,
    ffi::OsStr,
    fmt::Display,
    fs::{self, File},
    io::{Read, Seek, SeekFrom},
    marker::PhantomData,
    path::Path,
};

use plist_plus::Plist;
use rusty_libimobiledevice::services::{
    afc::{AfcClient, AfcFileMode},
    instproxy::InstProxyClient,
};
use zip::ZipArchive;

pub mod errors;

use crate::{device::DeviceClient, devices_collection::SingleDevice, errors::DeviceInstallerError};

const PKG_PATH: &str = "PublicStaging";
const IPCC_REMOTE_FOLDER: &str = "rsmobiledevice.ipcc";
const IPA_REMOTE_FILE: &str = "rsmobiledevice.ipa";

#[derive(Debug)]
pub struct DeviceInstaller<T> {
    device: DeviceClient<T>,
    _p: PhantomData<T>,
}

enum PackageType {
    #[allow(clippy::upper_case_acronyms)]
    IPCC,
    #[allow(clippy::upper_case_acronyms)]
    IPA,
    Unknown,
}

impl Display for PackageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageType::IPA => write!(f, "ipa"),
            PackageType::IPCC => write!(f, "ipcc"),
            PackageType::Unknown => write!(f, "unknown"),
        }
    }
}

impl DeviceInstaller<SingleDevice> {
    pub fn install_from_path<S>(
        &self,
        package_path: &S,
        options: Option<HashMap<&str, &str>>,
    ) -> Result<(), DeviceInstallerError>
    where
        S: AsRef<OsStr> + ?Sized,
    {
        let file = fs::File::open(Path::new(package_path.as_ref()))?;

        self._install_package(&file, options)
    }

    pub fn install_from_file_object(
        &self,
        package_file: &File,
        options: Option<HashMap<&str, &str>>,
    ) -> Result<(), DeviceInstallerError> {
        self._install_package(package_file, options)
    }

    fn _install_package(
        &self,
        file: &File,
        options: Option<HashMap<&str, &str>>,
    ) -> Result<(), DeviceInstallerError> {
        let device = self
            .device
            .get_device()
            .ok_or(DeviceInstallerError::DeviceNotFound)?;

        let afc_client = self.device.get_afc_client::<DeviceInstallerError>()?;

        self.check_or_create_path(&afc_client, PKG_PATH)?;

        let package_type = self.determine_file_package_type(file)?;

        let mut package_options = InstProxyClient::client_options_new();

        if let Some(dict) = options {
            for (key, value) in dict {
                package_options.dict_set_item(key, value.into())?;
            }
        }
        self.upload_package(&afc_client, &package_type, file)?;

        let installation_client = device.new_instproxy_client("rsmobiledevice-deviceinstaller")?;

        match package_type {
            PackageType::IPCC => {
                package_options.dict_set_item("PackageType", "CarrierBundle".into())?;
                installation_client.install(
                    format!("/{}/{}", PKG_PATH, IPA_REMOTE_FILE),
                    Some(package_options),
                )?;
            }

            PackageType::IPA => {
                let bundle_id = self.get_bundle_id(file)?;

                package_options.dict_set_item("CFBundleIdentifier", bundle_id.into())?;

                installation_client.install(
                    format!("/{}/{}", PKG_PATH, IPA_REMOTE_FILE),
                    Some(package_options),
                )?;
            }
            PackageType::Unknown => return Err(DeviceInstallerError::UnknownPackage),
        };

        Ok(())
    }

    fn determine_file_package_type(
        &self,
        package: &File,
    ) -> Result<PackageType, DeviceInstallerError> {
        let mut archive = ZipArchive::new(package)?;

        let mut package_type = PackageType::Unknown;

        let inside_file = archive.by_index(1)?;

        let file_path_string = inside_file
            .enclosed_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_owned();

        let splitted_file_path: Vec<&str> = file_path_string.split("/").collect();

        for path in splitted_file_path {
            if path.ends_with(".bundle") {
                package_type = PackageType::IPCC;
            } else if path.ends_with(".app") {
                package_type = PackageType::IPA;
            }
        }

        Ok(package_type)
    }

    fn upload_package(
        &self,
        afc_client: &AfcClient<'_>,
        package_type: &PackageType,
        package: &File,
    ) -> Result<(), DeviceInstallerError> {
        match package_type {
            PackageType::IPCC => {
                self.check_or_create_path(
                    afc_client,
                    &format!("/{}/{}", PKG_PATH, IPCC_REMOTE_FOLDER),
                )?;
                self.upload_ipcc_files(afc_client, package)?;
            }
            PackageType::IPA => self.upload_ipa_package(afc_client, package)?,
            PackageType::Unknown => (),
        }

        Ok(())
    }

    fn upload_ipa_package(
        &self,
        afc_client: &AfcClient<'_>,
        mut ipa_file: &File,
    ) -> Result<(), DeviceInstallerError> {
        let remote_file_handler = afc_client.file_open(
            format!("/{}/{}", PKG_PATH, IPA_REMOTE_FILE),
            AfcFileMode::WriteOnly,
        )?;
        // reset the cursor to the beginning for proper reading
        ipa_file.seek(SeekFrom::Start(0))?;

        let mut file_bytes = Vec::new();

        ipa_file.read_to_end(&mut file_bytes)?;

        afc_client.file_write(remote_file_handler, file_bytes)?;
        afc_client.file_close(remote_file_handler)?;

        Ok(())
    }

    fn upload_ipcc_files(
        &self,
        afc_client: &AfcClient<'_>,
        ipcc_file: &File,
    ) -> Result<(), DeviceInstallerError> {
        let mut archive = ZipArchive::new(ipcc_file)?;

        for i in 0..archive.len() {
            let mut inside_file = archive.by_index(i)?;
            let current_file_path = format!(
                "/{}/{}/{}",
                PKG_PATH,
                IPCC_REMOTE_FOLDER,
                inside_file
                    .enclosed_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            );

            if inside_file.is_dir() {
                afc_client.make_directory(current_file_path)?;
            } else {
                let mut inside_file_bytes = Vec::new();

                inside_file.read_to_end(&mut inside_file_bytes)?;

                let remote_file_handler =
                    afc_client.file_open(current_file_path, AfcFileMode::WriteOnly)?;

                afc_client.file_write(remote_file_handler, inside_file_bytes)?;
            }
        }

        Ok(())
    }
    fn get_bundle_id(&self, file: &File) -> Result<String, DeviceInstallerError> {
        let mut zip_file = ZipArchive::new(file).unwrap();

        let mut bundle_id = String::new();

        for i in 0..zip_file.len() {
            let mut file = zip_file.by_index(i)?;
            let inner_file_path = file.enclosed_name();

            if inner_file_path.is_none() {
                continue;
            }

            let inner_file_path = inner_file_path.unwrap();

            for path in inner_file_path.iter() {
                if path.to_str() == Some("Info.plist")
                    && inner_file_path.to_str().unwrap_or_default().len() == 3
                {
                    let mut plist_content = String::new();
                    file.read_to_string(&mut plist_content)?;

                    let plist = Plist::from_xml(plist_content)?;
                    if let Ok(bid) = plist
                        .dict_get_item("CFBundleIdentifier")
                        .and_then(|p| p.get_string_val())
                    {
                        bundle_id = bid;
                    }
                }
            }
        }
        Ok(bundle_id)
    }

    fn check_or_create_path(
        &self,
        afc_client: &AfcClient<'_>,
        path: &str,
    ) -> Result<(), DeviceInstallerError> {
        if afc_client.get_file_info(path).is_ok() {
            Ok(())
        } else {
            afc_client
                .make_directory(path)
                .map_err(|_| DeviceInstallerError::ErrorCreatingFolder)
        }
    }
}

impl<T> DeviceInstaller<T> {
    pub fn new(device: DeviceClient<T>) -> DeviceInstaller<T> {
        DeviceInstaller {
            device,
            _p: PhantomData::<T>,
        }
    }
}
