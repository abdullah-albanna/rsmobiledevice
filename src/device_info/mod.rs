use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;

pub mod domains;
pub mod errors;
pub mod keys;

use crate::device::DeviceClient;
use crate::devices_collection::{DeviceGroup, SingleDevice};
use domains::DeviceDomains;
use errors::DeviceInfoError;
use keys::DeviceKeys;
use plist_plus::Plist;

#[derive(Debug)]
pub struct DeviceInfo<T> {
    device: DeviceClient<T>,
    _p: PhantomData<T>,
}

impl Display for DeviceInfo<SingleDevice> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();

        let output = self
            .get_plist("", DeviceDomains::All)
            .expect("Couldn't display device info");

        for line in output.into_iter() {
            text.push_str(
                format!(
                    "{}: {}\n",
                    line.key.unwrap(),
                    line.plist.get_display_value().unwrap()
                )
                .as_str(),
            );
        }

        write!(f, "{}", text)
    }
}

impl Display for DeviceInfo<DeviceGroup> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();

        let plists = self
            .get_plist("", DeviceDomains::All)
            .expect("Couldn't display device info");

        for (i, plist) in plists.into_iter().enumerate() {
            text.push_str(format!("{}:\n", i + 1).as_str());
            for line in plist {
                text.push_str(
                    format!(
                        "\t{}: {}\n",
                        line.key.unwrap(),
                        line.plist.get_display_value().unwrap()
                    )
                    .as_str(),
                );
            }
        }

        write!(f, "{}", text)
    }
}
impl DeviceInfo<SingleDevice> {
    pub fn get_plist(
        &self,
        key: impl Into<String> + Copy,
        domain: DeviceDomains,
    ) -> Result<Plist, DeviceInfoError> {
        self.device.check_connected::<DeviceInfoError>()?;

        let lockdownd = self.device.get_lockdownd_client::<DeviceInfoError>()?;
        let output = lockdownd
            .get_value(key.into(), domain.as_string())
            .map_err(DeviceInfoError::LockdowndError)?;

        Ok(output)
    }

    pub fn get_values(
        &self,
        domain: DeviceDomains,
    ) -> Result<HashMap<String, String>, DeviceInfoError> {
        let mut dict: HashMap<String, String> = HashMap::new();

        let output = self.get_plist("", domain)?;

        for line in output {
            dict.insert(
                line.key.unwrap_or("unknown".to_string()),
                line.plist
                    .get_display_value()
                    .unwrap_or("unknown".to_string())
                    .replace('"', ""),
            );
        }
        Ok(dict)
    }

    pub fn get_value(
        &self,
        key: DeviceKeys,
        domain: DeviceDomains,
    ) -> Result<String, DeviceInfoError> {
        let values = self.get_values(domain)?;

        if let Some(key) = values.get(&key.to_string()) {
            Ok(key.to_owned())
        } else {
            Err(DeviceInfoError::KeyNotFound)
        }
    }

    pub fn get_all_values(&self) -> Result<HashMap<String, String>, DeviceInfoError> {
        self.get_values(DeviceDomains::All)
    }

    pub fn get_product_type(&self) -> String {
        self.get_value(DeviceKeys::ProductType, DeviceDomains::All)
            .expect("Couldn't get the product type, this is a bug")
    }

    pub fn get_product_version(&self) -> String {
        self.get_value(DeviceKeys::ProductType, DeviceDomains::All)
            .expect("Couldn't get the product version, this is a bug")
    }
}
impl DeviceInfo<DeviceGroup> {
    pub fn get_plist(
        &self,
        key: impl Into<String>,
        domain: DeviceDomains,
    ) -> Result<Vec<Plist>, DeviceInfoError> {
        let lockdownds = self.device.get_lockdownd_clients::<DeviceInfoError>()?;

        let key = key.into();

        let plists = lockdownds
            .into_iter()
            .map(|lockdownd| lockdownd.get_value(&key, domain.as_string()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(plists)
    }

    pub fn get_values(
        &self,
        domain: DeviceDomains,
    ) -> Result<Vec<HashMap<String, String>>, DeviceInfoError> {
        let mut dicts: Vec<HashMap<String, String>> = Vec::new();

        for plist in self.get_plist("", domain)?.into_iter() {
            let mut device_dict = HashMap::new();
            for line in plist {
                device_dict.insert(
                    line.key.unwrap_or("unknown".to_string()),
                    line.plist
                        .get_display_value()
                        .unwrap_or("unknown".to_string())
                        .replace('"', ""),
                );
            }

            dicts.push(device_dict);
        }

        Ok(dicts)
    }

    pub fn get_value(
        &self,
        key: DeviceKeys,
        domain: DeviceDomains,
    ) -> Result<Vec<String>, DeviceInfoError> {
        let values = self.get_values(domain)?;

        let mut selected_key_values = Vec::new();

        for value in values {
            if let Some(key) = value.get(&key.to_string()) {
                selected_key_values.push(key.to_owned())
            } else {
                return Err(DeviceInfoError::KeyNotFound);
            }
        }
        Ok(selected_key_values)
    }

    pub fn get_all_values(&self) -> Result<Vec<HashMap<String, String>>, DeviceInfoError> {
        self.get_values(DeviceDomains::All)
    }

    pub fn get_product_type(&self) -> Vec<String> {
        self.get_value(DeviceKeys::ProductType, DeviceDomains::All)
            .expect("Couldn't get the product type, this is a bug")
    }

    pub fn get_product_version(&self) -> Vec<String> {
        self.get_value(DeviceKeys::ProductType, DeviceDomains::All)
            .expect("Couldn't get the product version, this is a bug")
    }
}

impl<T> DeviceInfo<T> {
    pub fn new(device: DeviceClient<T>) -> DeviceInfo<T> {
        DeviceInfo {
            device,
            _p: PhantomData::<T>,
        }
    }
}
