//! Provides an interface for retrieving and displaying device information from iOS devices.
//!
//! ## Features
//! - Retrieves plist data from a connected device or group of devices
//! - Supports querying values based on device keys and domains

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

/// Struct for managing device information retrieval
///
/// # Type Parameters
/// - `T`: The type of the device or device group (SingleDevice or DeviceGroup)
///
#[derive(Debug)]
pub struct DeviceInfo<'a, T> {
    device: &'a DeviceClient<T>,
    _p: PhantomData<T>,
}

impl Display for DeviceInfo<'_, SingleDevice> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();

        let output = self
            .get_plist("", DeviceDomains::All)
            .expect("Couldn't display device info");

        // Format each line of the plist data
        for line in output {
            text.push_str(&format!(
                "{}: {}\n",
                line.key.unwrap_or("unknown".into()),
                line.plist.get_display_value().unwrap_or("unknown".into())
            ));
        }

        write!(f, "{}", text)
    }
}

impl Display for DeviceInfo<'_, DeviceGroup> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();

        let plists = self
            .get_plist_all("", DeviceDomains::All)
            .expect("Couldn't display device info");

        // Iterate over all the devices and format their information
        for (i, plist) in plists.into_iter().enumerate() {
            text.push_str(&format!("{}:\n", i + 1));
            for line in plist {
                text.push_str(&format!(
                    "\t{}: {}\n",
                    line.key.unwrap_or("unknown".into()),
                    line.plist.get_display_value().unwrap_or("unknown".into())
                ));
            }
        }

        write!(f, "{}", text)
    }
}

impl DeviceInfo<'_, SingleDevice> {
    /// Retrieves a plist from a single device.
    ///
    /// # Arguments
    /// - `key`: The specific key to query.
    /// - `domain`: The domain within which to search for the key.
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

    /// Retrieves multiple values from a device based on a domain.
    ///
    /// # Arguments
    /// - `domain`: The domain within which to retrieve all values.
    ///
    pub fn get_values(
        &self,
        domain: DeviceDomains,
    ) -> Result<HashMap<String, String>, DeviceInfoError> {
        self.device.check_connected::<DeviceInfoError>()?;
        let mut dict: HashMap<String, String> = HashMap::new();

        let output = self.get_plist("", domain)?;

        // Populate the HashMap with device data
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

    /// Retrieves a single value from a device based on a key and domain.
    ///
    /// # Arguments
    /// - `key`: The key to query.
    /// - `domain`: The domain within which to search for the key.
    pub fn get_value(
        &self,
        key: DeviceKeys,
        domain: DeviceDomains,
    ) -> Result<String, DeviceInfoError> {
        self.device.check_connected::<DeviceInfoError>()?;
        let values = self.get_values(domain)?;

        if let Some(key) = values.get(&key.to_string()) {
            Ok(key.to_owned())
        } else {
            Err(DeviceInfoError::KeyNotFound)
        }
    }

    /// Retrieves all the values from all the domains.
    pub fn get_all_values(&self) -> Result<HashMap<String, String>, DeviceInfoError> {
        self.device.check_connected::<DeviceInfoError>()?;
        self.get_values(DeviceDomains::All)
    }

    pub fn get_product_type(&self) -> Result<String, DeviceInfoError> {
        self.device.check_connected::<DeviceInfoError>()?;
        self.get_value(DeviceKeys::ProductType, DeviceDomains::All)
    }

    pub fn get_product_version(&self) -> Result<String, DeviceInfoError> {
        self.device.check_connected::<DeviceInfoError>()?;
        self.get_value(DeviceKeys::ProductVersion, DeviceDomains::All)
    }
}

impl DeviceInfo<'_, DeviceGroup> {
    /// Retrieves plist data for all devices in a group.
    ///
    /// # Arguments
    /// - `key`: The specific key to query.
    /// - `domain`: The domain within which to search for the key.
    pub fn get_plist_all(
        &self,
        key: impl Into<String>,
        domain: DeviceDomains,
    ) -> Result<Vec<Plist>, DeviceInfoError> {
        self.device.check_all_connected::<DeviceInfoError>()?;
        let lockdownds = self.device.get_lockdownd_clients::<DeviceInfoError>()?;

        let key = key.into();

        let plists = lockdownds
            .into_iter()
            .map(|lockdownd| lockdownd.get_value(&key, domain.as_string()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(plists)
    }

    /// Retrieves multiple values for all devices in a group based on a domain.
    ///
    /// # Arguments
    /// - `domain`: The domain within which to retrieve all values.
    pub fn get_values_all(
        &self,
        domain: DeviceDomains,
    ) -> Result<Vec<HashMap<String, String>>, DeviceInfoError> {
        self.device.check_all_connected::<DeviceInfoError>()?;
        let mut dicts: Vec<HashMap<String, String>> = Vec::new();

        for plist in self.get_plist_all("", domain)?.into_iter() {
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

    /// Retrieves a specific value for all devices in a group based on a key and domain.
    ///
    /// # Arguments
    /// - `key`: The key to query.
    /// - `domain`: The domain within which to search for the key.
    pub fn get_value_all(
        &self,
        key: DeviceKeys,
        domain: DeviceDomains,
    ) -> Result<Vec<String>, DeviceInfoError> {
        self.device.check_all_connected::<DeviceInfoError>()?;
        let values = self.get_values_all(domain)?;

        values
            .into_iter()
            .map(|value| {
                value
                    .get(&key.to_string())
                    .cloned() // Convert from &String to String
                    .ok_or(DeviceInfoError::KeyNotFound)
            })
            .collect::<Result<Vec<_>, _>>()
    }

    /// Retrieves all values for all devices in a group.
    pub fn get_all_values_all(&self) -> Result<Vec<HashMap<String, String>>, DeviceInfoError> {
        self.device.check_all_connected::<DeviceInfoError>()?;
        self.get_values_all(DeviceDomains::All)
    }

    /// Retrieves the product type for all connected device.
    pub fn get_product_type_all(&self) -> Result<Vec<String>, DeviceInfoError> {
        self.device.check_all_connected::<DeviceInfoError>()?;
        self.get_value_all(DeviceKeys::ProductType, DeviceDomains::All)
    }

    /// Retrieves the product version for all connected device.
    pub fn get_product_version_all(&self) -> Result<Vec<String>, DeviceInfoError> {
        self.device.check_all_connected::<DeviceInfoError>()?;
        self.get_value_all(DeviceKeys::ProductVersion, DeviceDomains::All)
    }
}

impl<'a, T> DeviceInfo<'a, T> {
    pub fn new(device: &'a DeviceClient<T>) -> DeviceInfo<'a, T> {
        DeviceInfo {
            device,
            _p: PhantomData::<T>,
        }
    }
}
