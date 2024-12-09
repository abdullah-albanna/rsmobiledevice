use rusty_libimobiledevice::{
    idevice,
    services::{afc::AfcClient, lockdownd::LockdowndClient},
};
use std::marker::PhantomData;

use crate::{
    devices_collection::{DeviceGroup, Devices, SingleDevice},
    errors::{
        AFCClientErrorTrait, DeviceClientError, DeviceNotFoundErrorTrait, LockdowndErrorTrait,
    },
};

#[derive(Debug, Clone)]
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
    pub fn get_device(&self) -> Option<&idevice::Device> {
        self.device.get_device()
    }

    pub fn get_afc_client<E: AFCClientErrorTrait + DeviceNotFoundErrorTrait>(
        &self,
    ) -> Result<AfcClient, E> {
        if let Some(device) = self.get_device() {
            let afc_client = AfcClient::start_service(device, "rsmobiledevice-afc_client")
                .map_err(E::afcclient_error)?;

            Ok(afc_client)
        } else {
            Err(E::device_not_found())
        }
    }

    pub fn get_lockdown_client<E: LockdowndErrorTrait>(&self) -> Result<LockdowndClient, E> {
        let device = self.get_device().expect("couldn't get the deviec");

        let lockdown = LockdowndClient::new(device, "deviceclient-lockdown-client")
            .map_err(|err| E::lockdown_error(err))?;
        Ok(lockdown)
    }

    pub fn check_connected<E: DeviceNotFoundErrorTrait>(&self) -> Result<(), E> {
        if let Some(device) = self.get_device() {
            if let Ok(connected_devices) = idevice::get_devices() {
                if connected_devices
                    .iter()
                    .any(|d| d.get_udid() == device.get_udid())
                {
                    return Ok(());
                }
            }
        }
        Err(E::device_not_found())
    }
    pub fn is_connected(&self) -> bool {
        if let Some(device) = self.get_device() {
            if let Ok(connected_devices) = idevice::get_devices() {
                return connected_devices
                    .iter()
                    .any(|d| d.get_udid() == device.get_udid());
            }
        }
        false
    }
}

impl DeviceClient<DeviceGroup> {
    pub fn get_first_device(self) -> Option<DeviceClient<SingleDevice>> {
        if let Devices::Multiple(device) = self.device {
            Some(DeviceClient {
                device: Devices::Single(device.first().unwrap().clone()),
                _p: PhantomData::<SingleDevice>,
            })
        } else {
            None
        }
    }

    pub fn get_devices(&self) -> Option<&Vec<idevice::Device>> {
        self.device.get_devices()
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
