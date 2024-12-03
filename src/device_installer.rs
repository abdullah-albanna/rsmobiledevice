use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    io::Read,
    marker::PhantomData,
    path::Path,
};

use rusty_libimobiledevice::services::{
    afc::{AfcClient, AfcFileMode},
    instproxy::InstProxyClient,
};
use zip::ZipArchive;

use crate::{device::DeviceClient, devices::SingleDevice, errors::DeviceInstallerError};

const PKG_PATH: &str = "PublicStaging";

#[derive(Debug, Clone)]
pub struct DeviceInstaller<T> {
    devices: DeviceClient<T>,
    _p: PhantomData<T>,
}

enum PackageType {
    IPCC,
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
    pub fn install_from_path(
        &self,
        package_path: &Path,
        options: Option<HashMap<&str, &str>>,
    ) -> Result<(), DeviceInstallerError> {
        let file = fs::File::open(package_path)?;

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
            .devices
            .get_device()
            .ok_or(DeviceInstallerError::DeviceNotFound)?;

        let afc_client = self.devices.get_afc_client().unwrap();

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
                    format!("/{}/rsmobiledevice.ipcc", PKG_PATH),
                    Some(package_options),
                )?;
            }

            PackageType::IPA => {
                package_options.dict_set_item("PackageType", "Developer".into())?;
                installation_client.install(
                    format!("/{}/rsmobiledevice.ipa", PKG_PATH),
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
                    &format!("/{}/rsmobiledevice.ipcc", PKG_PATH),
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
            format!("/{}/rsmobiledevice.ipa", PKG_PATH),
            AfcFileMode::WriteOnly,
        )?;

        let mut file_bytes = Vec::new();

        ipa_file.read_to_end(&mut file_bytes)?;

        afc_client.file_write(remote_file_handler, file_bytes)?;
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

            if inside_file.is_dir() {
                afc_client.make_directory(
                    inside_file
                        .enclosed_name()
                        .unwrap_or_default()
                        .to_string_lossy(),
                )?;
            } else {
                let mut inside_file_bytes = Vec::new();

                inside_file.read_to_end(&mut inside_file_bytes)?;

                let remote_file_handler = afc_client.file_open(
                    inside_file
                        .enclosed_name()
                        .unwrap_or_default()
                        .to_string_lossy(),
                    AfcFileMode::WriteOnly,
                )?;

                afc_client.file_write(remote_file_handler, inside_file_bytes)?;
            }
        }

        Ok(())
    }
    //fn get_bundle_id(&self, &mut zip_file: &ZipArchive<&File>) -> Option<String> {
    //    let info_re = regex::Regex::new("Payload/[^/]*/Info.plist")
    //        .expect("Couldn't make a new regex, this is a bug");

    //    let mut bundle_id = String::new();
    //    for i in 0..zip_file.len() {
    //        if let Ok(mut file) = zip_file.by_index(i) {
    //            if let Some(output) = file.enclosed_name() {
    //                if info_re.is_match(output.to_str().unwrap_or_default()) {
    //                    let mut buffer = Vec::new();
    //                    if io::copy(&mut file, &mut buffer).is_err() {
    //                        return None;
    //                    }
    //                    if let Ok(plist) = Plist::from_bin(buffer) {
    //                        if let Ok(id) = plist
    //                            .dict_get_item("CFBundleIdentifier")
    //                            .and_then(|id| id.get_string_val())
    //                        {
    //                            bundle_id = id;
    //                        } else {
    //                            return None;
    //                        }
    //                    } else {
    //                        return None;
    //                    }
    //                }
    //            }
    //        }
    //    }

    //    Some(bundle_id)
    //}

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
    pub fn new(devices: DeviceClient<T>) -> DeviceInstaller<T> {
        DeviceInstaller {
            devices,
            _p: PhantomData::<T>,
        }
    }
}
