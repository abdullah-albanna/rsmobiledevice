//! The core module for all other services

//! ## Overview
//!
//! the `DeviceClient` holds in the actual client for the connected device/s.
//!
//! You must create one to get anything else

use rusty_libimobiledevice::{
    callback::IDeviceEventCallback,
    idevice::{self, EventType},
    services::{afc::AfcClient, lockdownd::LockdowndClient},
};
use std::{any::Any, marker::PhantomData};

use crate::{
    device_diagnostic::DeviceDiagnostic,
    device_info::DeviceInfo,
    device_installer::DeviceInstaller,
    device_syslog::DeviceSysLog,
    devices_collection::{DeviceGroup, Devices, SingleDevice},
    errors::{
        AfcClientErrorTrait, DeviceClientError, DeviceNotFoundErrorTrait, LockdowndErrorTrait,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    Connect,
    Disconnect,
    Pair,
}

impl From<EventType> for Event {
    fn from(value: EventType) -> Self {
        match value {
            EventType::Add => Self::Connect,
            EventType::Remove => Self::Disconnect,
            EventType::Pair => Self::Pair,
        }
    }
}

pub fn event_subscribe<F>(mut cb: F) -> Result<(), DeviceClientError>
where
    F: FnMut(Event) + Send + Sync + 'static,
{
    Ok(rusty_libimobiledevice::idevice::event_subscribe(
        IDeviceEventCallback::new(
            Box::new(move |event, _| {
                cb(event.event_type().into());
            }),
            Box::new(0),
            None,
        ),
    )?)
}

pub fn event_unsubscribe() -> Result<(), DeviceClientError> {
    Ok(rusty_libimobiledevice::idevice::event_unsubscribe()?)
}

/// A high-level abstraction for managing iOS devices, generic over `T`.
///
/// - `T = SingleDevice`: For single-device operations.
/// - `T = DeviceGroup`: For operations involving multiple devices.
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

    /// Retrieves the underlying `idevice::Device` instance.
    ///
    /// # Panics
    /// This method will panic if the device is not found. This should never occur.
    pub fn get_device(&self) -> &idevice::Device {
        self.device
            .get_device()
            .expect("Unexpected error: Device not found.")
    }

    /// Creates an `AfcClient` for file management operations.
    ///
    /// # Errors
    /// Returns an error if the device is not connected or if the AFC service fails to start.
    pub(crate) fn get_dynamic_afc_client<E: AfcClientErrorTrait + DeviceNotFoundErrorTrait>(
        &self,
    ) -> Result<AfcClient, E> {
        self.check_connected()?;
        let device = self.get_device();
        AfcClient::start_service(device, "rsmobiledevice-afc_client").map_err(E::afcclient_error)
    }

    /// Creates a `LockdowndClient` for interacting with device services.
    ///
    /// # Errors
    /// Returns an error if the device is not connected or if the lockdownd service fails.
    pub(crate) fn get_dynamic_lockdownd_client<
        E: LockdowndErrorTrait + DeviceNotFoundErrorTrait,
    >(
        &self,
    ) -> Result<LockdowndClient, E> {
        self.check_connected()?;
        let device = self.get_device();
        LockdowndClient::new(device, "rsmobiledevice-lockdownd-client")
            .map_err(|err| E::lockdownd_error(err))
    }

    /// Verifies that the device is currently connected, if it was not found, that means
    /// that the device that was connected when the client was created is no longer connected.
    ///
    /// # Errors
    /// Returns an error if the device is not found in the list of connected device.
    pub fn check_connected<E: DeviceNotFoundErrorTrait>(&self) -> Result<(), E> {
        let device = self.get_device();
        let connected_devices = idevice::get_devices().unwrap_or_default();
        if connected_devices
            .iter()
            .any(|d| d.get_udid() == device.get_udid())
        {
            Ok(())
        } else {
            Err(E::device_not_found())
        }
    }

    /// Checks whether the device is connected.
    ///
    /// This is used for if conditions, rather than returning an error
    pub fn is_connected(&self) -> bool {
        let device = self.get_device();
        let connected_devices = idevice::get_devices().unwrap_or_default();
        connected_devices
            .iter()
            .any(|d| d.get_udid() == device.get_udid())
    }

    pub fn watch_device<F>(&self, mut cb: F) -> Result<(), DeviceClientError>
    where
        F: FnMut(Event) + Send + Sync + 'static,
    {
        rusty_libimobiledevice::idevice::event_subscribe(IDeviceEventCallback::new(
            Box::new(move |event, _| {
                let event = event.event_type().into();
                cb(event);
            }),
            Box::new(0),
            Some(self.device.get_device().unwrap().get_udid()),
        ))?;

        Ok(())
    }
}

impl DeviceClient<DeviceGroup> {
    /// Retrieves the first available device in the group, if any.
    pub fn get_first_device(self) -> Option<DeviceClient<SingleDevice>> {
        self.get_devices().first().map(|first_device| DeviceClient {
            device: Devices::Single(first_device.to_owned()),
            _p: PhantomData::<SingleDevice>,
        })
    }

    /// Retrieves a list of all the devices that were connected when the client was created
    pub fn get_devices(&self) -> &Vec<idevice::Device> {
        self.device
            .get_devices()
            .expect("Unexpected error: No devices found.")
    }

    /// Creates `AfcClient` instances for all connected devices.
    ///
    /// # Errors
    /// Returns an error if any device is not connected or if the AFC service fails for a device.
    pub fn get_afc_clients<E: AfcClientErrorTrait + DeviceNotFoundErrorTrait>(
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

    /// Creates `LockdowndClient` instances for all connected devices.
    ///
    /// # Errors
    /// Returns an error if any device is not connected or if the lockdownd service fails.
    pub fn get_lockdownd_clients<E: LockdowndErrorTrait + DeviceNotFoundErrorTrait>(
        &self,
    ) -> Result<Vec<LockdowndClient>, E> {
        self.check_all_connected()?;
        self.get_devices()
            .iter()
            .map(|device| {
                LockdowndClient::new(device, "rsmobiledevice-lockdownd-clients")
                    .map_err(E::lockdownd_error)
            })
            .collect()
    }

    /// Verifies that all devices in the group are connected.
    ///
    /// # Errors
    /// Returns an error if any device is not found in the list of connected devices.
    pub fn check_all_connected<E: DeviceNotFoundErrorTrait>(&self) -> Result<(), E> {
        let connected_devices = idevice::get_devices().unwrap_or_default();
        let connected_udids: Vec<String> = connected_devices.iter().map(|d| d.get_udid()).collect();
        if self
            .get_devices()
            .iter()
            .all(|device| connected_udids.contains(&device.get_udid()))
        {
            Ok(())
        } else {
            Err(E::device_not_found())
        }
    }

    /// Checks whether all devices in the group are connected.
    pub fn are_connected(&self) -> bool {
        let connected_devices = idevice::get_devices().unwrap_or_default();
        let connected_udids: Vec<String> = connected_devices.iter().map(|d| d.get_udid()).collect();
        self.get_devices()
            .iter()
            .all(|device| connected_udids.contains(&device.get_udid()))
    }
}

impl TryFrom<String> for DeviceClient {
    type Error = DeviceClientError;

    /// Creates a `DeviceClient` from a device's unique identifier (UDID).
    ///
    /// # Errors
    /// Returns an error if the device with the specified UDID is not found.
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let device = idevice::get_device(value)?;
        Ok(Self {
            device: Devices::Single(device),
            _p: PhantomData,
        })
    }
}
