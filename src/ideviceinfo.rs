use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;

use crate::device_domains::DeviceDomains;
use crate::errors::IDeviceErrors;
use plist_plus::Plist;
use rusty_libimobiledevice;

use rusty_libimobiledevice::error::LockdowndError;
use rusty_libimobiledevice::idevice::{self, Device};
use rusty_libimobiledevice::services::lockdownd::LockdowndClient;

pub struct SingleDevice();
pub struct DeviceGroup();

#[derive(Debug, Clone)]
pub enum Devices {
    Single(idevice::Device),
    Multiple(Vec<idevice::Device>),
}

impl Devices {
    pub fn get_device(&self) -> Option<Device> {
        if let Devices::Single(device) = self {
            Some(device.clone())
        } else {
            None
        }
    }

    pub fn get_devices(&self) -> Option<Vec<Device>> {
        if let Devices::Multiple(devices) = self {
            Some(devices.clone())
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct IDeviceInfo<T = DeviceGroup> {
    idevices: Devices,
    _p: PhantomData<T>,
}

impl Display for IDeviceInfo<SingleDevice> {
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
                    line.plist.clone().get_display_value().unwrap()
                )
                .as_str(),
            );
        }

        write!(f, "{}", text)
    }
}

impl Display for IDeviceInfo<DeviceGroup> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = String::new();

        let plists = self
            .get_plist("", DeviceDomains::All)
            .expect("Couldn't display device info");

        for (i, plist) in plists.iter().enumerate() {
            text.push_str(format!("{}:\n", i + 1).as_str());
            for line in plist.clone() {
                text.push_str(
                    format!(
                        "\t{}: {}\n",
                        line.key.unwrap(),
                        line.plist.clone().get_display_value().unwrap()
                    )
                    .as_str(),
                );
            }
        }

        write!(f, "{}", text)
    }
}
impl IDeviceInfo<SingleDevice> {
    pub fn get_plist(
        &self,
        key: impl Into<String> + Copy,
        domain: DeviceDomains,
    ) -> Result<Plist, IDeviceErrors> {
        let device = self.idevices.get_device().unwrap();
        let lockdownd = device.new_lockdownd_client("rsmobiledevice-singledevice")?;
        let output = lockdownd.get_value(key.into(), domain.as_string())?;

        Ok(output)
    }

    pub fn get_dict(
        &self,
        key: impl Into<String> + Copy,
        domain: DeviceDomains,
    ) -> Result<HashMap<String, String>, IDeviceErrors> {
        let mut dict: HashMap<String, String> = HashMap::new();

        let output = self.get_plist(key, domain)?;

        for line in output {
            dict.insert(
                line.key.unwrap_or("unknown".to_string()),
                line.plist
                    .clone()
                    .get_display_value()
                    .unwrap_or("unknown".to_string())
                    .replace('"', ""),
            );
        }
        Ok(dict)
    }

    pub fn get_dict_all(&self) -> Result<HashMap<String, String>, IDeviceErrors> {
        self.get_dict("", DeviceDomains::All)
    }
}
impl IDeviceInfo<DeviceGroup> {
    pub fn get_plist(
        &self,
        key: impl Into<String> + Copy,
        domain: DeviceDomains,
    ) -> Result<Vec<Plist>, IDeviceErrors> {
        let devices = self.idevices.get_devices().unwrap();

        let lockdownds: Vec<Result<LockdowndClient<'_>, LockdowndError>> = devices
            .iter()
            .map(|device| device.new_lockdownd_client("rsmobiledevice-devicegroup"))
            .collect();

        let mut success_lockdownds = Vec::new();

        for lockdownd in lockdownds {
            match lockdownd {
                Ok(lockdown) => success_lockdownds.push(lockdown),
                Err(err) => return Err(IDeviceErrors::LockdowndError(err)),
            }
        }

        let plists: Vec<Result<Plist, LockdowndError>> = success_lockdownds
            .iter()
            .map(|lockdown| lockdown.get_value(key.into(), domain.as_string()))
            .collect();

        let mut success_plists = Vec::new();

        for plist in plists {
            match plist {
                Ok(p) => success_plists.push(p),
                Err(err) => return Err(IDeviceErrors::LockdowndError(err)),
            }
        }

        Ok(success_plists)
    }

    pub fn get_first_device(self) -> Option<IDeviceInfo<SingleDevice>> {
        if let Devices::Multiple(device) = self.idevices {
            Some(IDeviceInfo {
                idevices: Devices::Single(device.first().unwrap().clone()),
                _p: PhantomData::<SingleDevice>,
            })
        } else {
            None
        }
    }

    pub fn get_dict(
        &self,
        key: impl Into<String> + Copy,
        domain: DeviceDomains,
    ) -> Result<HashMap<u32, HashMap<String, String>>, IDeviceErrors> {
        let mut dicts: HashMap<u32, HashMap<String, String>> = HashMap::new();

        for (i, plist) in self.get_plist(key, domain)?.iter().enumerate() {
            let mut device_dict = HashMap::new();
            for line in plist.clone() {
                device_dict.insert(
                    line.key.unwrap_or("unknown".to_string()),
                    line.plist
                        .clone()
                        .get_display_value()
                        .unwrap_or("unknown".to_string())
                        .replace('"', ""),
                );
            }

            dicts.insert((i + 1) as u32, device_dict);
        }

        Ok(dicts)
    }

    pub fn get_dict_all(&self) -> Result<HashMap<u32, HashMap<String, String>>, IDeviceErrors> {
        self.get_dict("", DeviceDomains::All)
    }
}

impl IDeviceInfo {
    pub fn new() -> Result<IDeviceInfo<DeviceGroup>, IDeviceErrors> {
        let devices = idevice::get_devices()?;

        Ok(IDeviceInfo {
            idevices: Devices::Multiple(devices),
            _p: PhantomData::<DeviceGroup>,
        })
    }
}

impl TryFrom<String> for IDeviceInfo {
    type Error = IDeviceErrors;

    /// from udid
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let device = idevice::get_device(value)?;
        Ok(Self {
            idevices: Devices::Single(device),
            _p: PhantomData,
        })
    }
}
