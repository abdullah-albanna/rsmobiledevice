use rusty_libimobiledevice::{
    idevice,
    services::{afc::AfcClient, lockdownd::LockdowndClient},
};
use std::marker::PhantomData;

use crate::{
    device_diagnostic::DeviceDiagnostic,
    device_info::DeviceInfo,
    device_installer::DeviceInstaller,
    device_syslog::DeviceSysLog,
    devices_collection::{DeviceGroup, Devices, SingleDevice},
    errors::{
        AFCClientErrorTrait, DeviceClientError, DeviceNotFoundErrorTrait, LockdowndErrorTrait,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceClient<T = DeviceGroup> {
    device: Devices,
    _p: PhantomData<T>,
}

impl DeviceClient {
    pub fn new() -> Result<DeviceClient<DeviceGroup>, DeviceClientError> {
        let device = idevice::get_devices()?;

        Ok(DeviceClient {
            device: Devices::Multiple(device),
            _p: PhantomData::<DeviceGroup>,
        })
    }
}

impl DeviceClient<SingleDevice> {
    pub fn get_device_info(&self) -> DeviceInfo<'_, SingleDevice> {
        DeviceInfo::new(self)
    }
    pub fn get_device_diagnostic(&self) -> DeviceDiagnostic<'_, SingleDevice> {
        DeviceDiagnostic::new(self)
    }
    pub fn get_device_syslog(self) -> DeviceSysLog<SingleDevice> {
        DeviceSysLog::new(self)
    }
    pub fn get_device_installer(&self) -> DeviceInstaller<'_, SingleDevice> {
        DeviceInstaller::new(self)
    }
    pub fn get_device(&self) -> &idevice::Device {
        // this should never fail because this method only appear in a single device, thus we can
        // get the device
        self.device
            .get_device()
            .expect("Couldn't get the device, this is a bug, please report")
    }

    pub fn get_afc_client<E: AFCClientErrorTrait + DeviceNotFoundErrorTrait>(
        &self,
    ) -> Result<AfcClient, E> {
        self.check_connected()?;
        let device = self.get_device();

        let afc_client = AfcClient::start_service(device, "rsmobiledevice-afc_client")
            .map_err(E::afcclient_error)?;

        Ok(afc_client)
    }

    pub fn get_lockdownd_client<E: LockdowndErrorTrait + DeviceNotFoundErrorTrait>(
        &self,
    ) -> Result<LockdowndClient, E> {
        self.check_connected()?;
        let device = self.get_device();

        let lockdownd = LockdowndClient::new(device, "deviceclient-lockdownd-client")
            .map_err(|err| E::lockdownd_error(err))?;
        Ok(lockdownd)
    }

    pub fn check_connected<E: DeviceNotFoundErrorTrait>(&self) -> Result<(), E> {
        let device = self.get_device();

        if let Ok(connected_devices) = idevice::get_devices() {
            if connected_devices
                .iter()
                .any(|d| d.get_udid() == device.get_udid())
            {
                return Ok(());
            }
        }
        Err(E::device_not_found())
    }

    pub fn is_connected(&self) -> bool {
        let device = self.get_device();
        if let Ok(connected_devices) = idevice::get_devices() {
            return connected_devices
                .iter()
                .any(|d| d.get_udid() == device.get_udid());
        }
        false
    }
}

impl DeviceClient<DeviceGroup> {
    pub fn get_first_device(self) -> Option<DeviceClient<SingleDevice>> {
        let devices = self.get_devices();

        devices.first().map(|first_device| {
            Some(DeviceClient {
                device: Devices::Single(first_device.to_owned()),
                _p: PhantomData::<SingleDevice>,
            })
        })?
    }

    pub fn get_devices(&self) -> &Vec<idevice::Device> {
        // this should never fail because this method only appear in a device group, thus we can
        // get the devices
        self.device
            .get_devices()
            .expect("Couldn't get the devices, this is a bug, please report")
    }

    pub fn get_afc_clients<E: AFCClientErrorTrait + DeviceNotFoundErrorTrait>(
        &self,
    ) -> Result<Vec<AfcClient>, E> {
        self.check_all_connected()?;
        self.get_devices()
            .iter()
            .map(|device| {
                AfcClient::start_service(device, "rsmobiledevice-afc_clients")
                    .map_err(E::afcclient_error)
            })
            .collect()
    }

    pub fn get_lockdownd_clients<E: LockdowndErrorTrait + DeviceNotFoundErrorTrait>(
        &self,
    ) -> Result<Vec<LockdowndClient>, E> {
        self.check_all_connected()?;
        self.get_devices()
            .iter()
            .map(|device| {
                LockdowndClient::new(device, "deviceclient-lockdownd-clients")
                    .map_err(E::lockdownd_error)
            })
            .collect()
    }

    pub fn check_all_connected<E: DeviceNotFoundErrorTrait>(&self) -> Result<(), E> {
        if let Ok(connected_devices) = idevice::get_devices() {
            let connected_udids: Vec<String> =
                connected_devices.iter().map(|d| d.get_udid()).collect();

            if self
                .get_devices()
                .iter()
                .all(|device| connected_udids.contains(&device.get_udid()))
            {
                return Ok(());
            }
        }
        Err(E::device_not_found())
    }

    pub fn are_connected(&self) -> bool {
        if let Ok(connected_devices) = idevice::get_devices() {
            let connected_udids: Vec<String> =
                connected_devices.iter().map(|d| d.get_udid()).collect();

            return self
                .get_devices()
                .iter()
                .all(|device| connected_udids.contains(&device.get_udid()));
        }
        false
    }
}
impl TryFrom<String> for DeviceClient {
    type Error = DeviceClientError;

    /// Attempts to create an `DeviceInfo` instance from a given UDID string.
    ///
    /// This implementation converts a UDID (Unique Device Identifier) represented as a `String`
    /// into an `DeviceInfo` instance by retrieving the corresponding device using the `idevice` library.
    ///
    /// # Parameters
    ///
    /// - `value`: A `String` representing the UDID of the device.
    ///
    /// # Returns
    ///
    /// - `Ok(DeviceInfo)` if the device is successfully found and instantiated.
    /// - `Err(IDeviceErrors)` if there is an error retrieving the device (e.g., device not found or connection error).
    ///
    /// # Errors
    ///
    /// This function will return an error if the device corresponding to the provided UDID cannot be retrieved.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::DeviceInfo;
    /// use std::convert::TryFrom;
    ///
    /// let udid = "example-udid-string".to_string();
    /// match DeviceInfo::try_from(udid) {
    ///     Ok(device_info) => println!("Successfully created DeviceInfo: {:?}", device_info),
    ///     Err(err) => println!("Error: {:?}", err),
    /// }
    /// ```
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let device = idevice::get_device(value)?;
        Ok(Self {
            device: Devices::Single(device),
            _p: PhantomData,
        })
    }
}
