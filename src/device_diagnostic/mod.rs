//! This module provides functionality for device diagnostics, such as restarting and sleeping

use crate::{
    device::DeviceClient,
    devices_collection::{DeviceGroup, SingleDevice},
};
use enums::{DevicePowerAction, DiagnosticBehavior, DiagnosticType, IORegPlane};
use errors::DeviceDiagnosticError;
use plist_plus::Plist;
use rusty_libimobiledevice::services::{
    diagnostics_relay::DiagnosticsRelay, lockdownd::LockdowndService,
};
use std::marker::PhantomData;

pub mod enums;
pub(crate) mod errors;

const DIAGNOSTICS_RELAY_SERVICE: &str = "com.apple.mobile.diagnostics_relay";

#[allow(dead_code)]
const DIAGNOSTICS_RELAY_SERVICE_OLD: &str = "com.apple.iosdiagnostics.relay";

/// Represents a diagnostic interface for a device.
///
/// This struct allows performing diagnostic operations on a device,
/// such as querying the IORegistry, retrieving battery information,
/// and executing power actions like sleep, restart, or shutdown.
///
/// # Type Parameters
/// - `T`: Marker type indicating whether the diagnostic is for a single device or a group of devices.
///
#[derive(Debug)]
pub struct DeviceDiagnostic<'a, T> {
    device: &'a DeviceClient<T>,
    _phantom: PhantomData<T>,
}

impl<'a, T> DeviceDiagnostic<'a, T> {
    pub fn new(device: &'a DeviceClient<T>) -> DeviceDiagnostic<'a, T> {
        DeviceDiagnostic {
            device,
            _phantom: PhantomData::<T>,
        }
    }
}

impl DeviceDiagnostic<'_, SingleDevice> {
    /// Retrieves the diagnostics relay service for the device.
    ///
    /// This internal method establishes a connection to the device's diagnostics relay service.
    ///
    fn get_diagnostic_relay(&self) -> Result<DiagnosticsRelay, DeviceDiagnosticError> {
        let device = self.device.get_device();
        let mut lockdownd = self
            .device
            .get_lockdownd_client::<DeviceDiagnosticError>()?;
        let diagnostic_service = lockdownd
            .start_service(DIAGNOSTICS_RELAY_SERVICE, true)
            .map_err(|e| DeviceDiagnosticError::ServiceError(e.to_string()))?;
        let relay = DiagnosticsRelay::new(device, diagnostic_service)
            .map_err(|e| DeviceDiagnosticError::RelayInitializationError(e.to_string()))?;
        Ok(relay)
    }

    /// Performs a power action on the device.
    ///
    /// This internal method sends a power action command to the device, such as sleep, restart, or shutdown.
    ///
    /// # Arguments
    /// - `action`: The power action to perform.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the action cannot be completed.
    fn device_power_action(&self, action: DevicePowerAction) -> Result<(), DeviceDiagnosticError> {
        let relay = self.get_diagnostic_relay()?;
        match action {
            DevicePowerAction::Sleep => relay.sleep()?,
            DevicePowerAction::Restart(flag) => relay.restart(flag as core::ffi::c_uint)?,
            DevicePowerAction::Shutdown(flag) => relay.shutdown(flag as core::ffi::c_uint)?,
        }
        Ok(())
    }

    /// Queries the IORegistry plane of the device.
    ///
    /// Retrieves information from the specified IORegistry plane.
    ///
    /// # Arguments
    /// - `plane`: The IORegistry plane to query.
    ///
    /// # Returns
    /// A `Plist` containing the queried information.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the query fails.
    pub fn query_ioreg_plane(&self, plane: IORegPlane) -> Result<Plist, DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        let relay = self.get_diagnostic_relay()?;
        Ok(relay.query_ioregistry_plane(plane.to_string())?)
    }

    /// Queries a specific IORegistry entry by key.
    ///
    /// Retrieves information associated with the specified key from the IORegistry.
    ///
    /// # Arguments
    /// - `key`: The key of the IORegistry entry to query.
    ///
    /// # Returns
    /// A `Plist` containing the queried information.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the query fails.
    pub fn query_ioregentry_key(
        &self,
        key: impl Into<String>,
    ) -> Result<Plist, DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        let relay = self.get_diagnostic_relay()?;
        Ok(relay.query_ioregistry_entry(key, "")?)
    }

    /// Queries the device for specific MobileGestalt keys.
    ///
    /// Retrieves values for the specified MobileGestalt keys.
    ///
    /// # Arguments
    /// - `keys`: A vector of keys to query.
    ///
    /// # Returns
    /// A `Plist` containing the key-value pairs.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the query fails.
    pub fn query_mobilegestalt(
        &self,
        keys: Vec<impl Into<String>>,
    ) -> Result<Plist, DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        let relay = self.get_diagnostic_relay()?;
        let mut plist = Plist::new_array();
        for (i, key) in keys.into_iter().enumerate() {
            plist.array_insert_item(Plist::new_string(&(key.into())), i as u32)?;
        }
        Ok(relay.query_mobilegestalt(plist)?)
    }

    /// Requests diagnostic information from the device.
    ///
    /// Retrieves diagnostics data of the specified type.
    ///
    /// # Arguments
    /// - `type`: The type of diagnostics to request.
    ///
    /// # Returns
    /// A `Plist` containing the diagnostics information.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the request fails.
    pub fn query_diagnostics(
        &self,
        r#type: DiagnosticType,
    ) -> Result<Plist, DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;

        let relay = self.get_diagnostic_relay()?;
        Ok(relay.request_diagnostics(r#type.to_string())?)
    }

    /// Retrieves battery-related information as a `Plist`.
    ///
    /// This method queries the device's IORegistry for battery information.
    /// For older devices (iPhone 7 or earlier), the `AppleARMPMUCharger` key is used.
    /// For newer devices, the `AppleSmartBattery` key is queried.
    ///
    /// # Returns
    /// A `Plist` containing battery-related information.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the query fails or the device information cannot be retrieved.
    pub fn get_battery_plist(&self) -> Result<Plist, DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        let product_version = self
            .device
            .get_device_info()
            .get_product_type()
            .map_err(|_| DeviceDiagnosticError::DeviceNotFound)?
            .strip_prefix("iPhone")
            .map_or(0, |s| {
                s.split(",")
                    .next()
                    .map_or(0, |n| n.parse::<u32>().unwrap_or_default())
            });

        if product_version <= 9 {
            // Applies only to iPhone 7 and earlier
            self.query_ioregentry_key("AppleARMPMUCharger")
        } else {
            self.query_ioregentry_key("AppleSmartBattery")
        }
    }

    /// Puts the device to sleep.
    ///
    /// Sends a command to the device to enter sleep mode.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the action fails.
    pub fn sleep(&self) -> Result<(), DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        self.device_power_action(DevicePowerAction::Sleep)
    }

    /// Restarts the device.
    ///
    /// Sends a command to the device to restart. The behavior can be customized using the `flag` parameter.
    ///
    /// # Arguments
    /// - `flag`: Defines the diagnostic behavior for the restart action.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the action fails.
    pub fn restart(&self, flag: DiagnosticBehavior) -> Result<(), DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        self.device_power_action(DevicePowerAction::Restart(flag))
    }

    /// Shuts down the device.
    ///
    /// Sends a command to the device to shut down. The behavior can be customized using the `flag` parameter.
    ///
    /// # Arguments
    /// - `flag`: Defines the diagnostic behavior for the shutdown action.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the action fails.
    pub fn shutdown(&self, flag: DiagnosticBehavior) -> Result<(), DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        self.device_power_action(DevicePowerAction::Shutdown(flag))
    }
}

impl DeviceDiagnostic<'_, DeviceGroup> {
    /// Retrieves diagnostics relay services for all devices in the group.
    ///
    /// Establishes connections to the diagnostics relay services for each device in the group.
    ///
    /// # Returns
    /// A vector of `DiagnosticsRelay` instances.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if any service cannot be started or any relay cannot be initialized.
    fn get_diagnostic_relaies(&self) -> Result<Vec<DiagnosticsRelay>, DeviceDiagnosticError> {
        let devices = self.device.get_devices();
        let mut lockdownds = self
            .device
            .get_lockdownd_clients::<DeviceDiagnosticError>()?;

        let diagnostic_services: Vec<LockdowndService> = lockdownds
            .iter_mut()
            .map(|lockdownd| {
                lockdownd
                    .start_service(DIAGNOSTICS_RELAY_SERVICE, true)
                    .map_err(|err| DeviceDiagnosticError::ServiceError(err.to_string()))
            })
            .collect::<Result<Vec<_>, DeviceDiagnosticError>>()?;

        diagnostic_services
            .into_iter()
            .zip(devices.iter())
            .map(|(service, device)| {
                DiagnosticsRelay::new(device, service)
                    .map_err(|err| DeviceDiagnosticError::RelayInitializationError(err.to_string()))
            })
            .collect()
    }

    /// Performs a power action on all devices in the group.
    ///
    /// Sends a power action command to each device in the group, such as sleep, restart, or shutdown.
    ///
    /// # Arguments
    /// - `action`: The power action to perform.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if any action fails.
    fn devices_power_action(&self, action: DevicePowerAction) -> Result<(), DeviceDiagnosticError> {
        let relays = self.get_diagnostic_relaies()?;
        for relay in relays {
            match action {
                DevicePowerAction::Sleep => relay.sleep()?,
                DevicePowerAction::Restart(flag) => relay.restart(flag as core::ffi::c_uint)?,
                DevicePowerAction::Shutdown(flag) => relay.shutdown(flag as core::ffi::c_uint)?,
            }
        }
        Ok(())
    }

    /// Queries the IORegistry plane for all devices in the group.
    ///
    /// # Arguments
    /// - `plane`: The IORegistry plane to query.
    ///
    /// # Returns
    /// A vector of `Plist` containing the queried information for each device.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the query fails for any device.
    pub fn query_ioreg_plane_all(
        &self,
        plane: IORegPlane,
    ) -> Result<Vec<Plist>, DeviceDiagnosticError> {
        self.device.check_all_connected::<DeviceDiagnosticError>()?;
        let relays = self.get_diagnostic_relaies()?;

        Ok(relays
            .into_iter()
            .map(|relay| relay.query_ioregistry_plane(plane.to_string()))
            .collect::<Result<Vec<_>, _>>()?)
    }

    /// Queries MobileGestalt information for all devices in the group.
    ///
    /// # Arguments
    /// - `keys`: A vector of keys to query.
    ///
    /// # Returns
    /// A vector of `Plist` objects containing the queried MobileGestalt information for each device.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the query fails for any device.
    pub fn query_mobilegestalt_all(
        &self,
        keys: Vec<impl Into<String>>,
    ) -> Result<Vec<Plist>, DeviceDiagnosticError> {
        self.device.check_all_connected::<DeviceDiagnosticError>()?;
        let relays = self.get_diagnostic_relaies()?;
        let mut plist = Plist::new_array();

        for (i, key) in keys.into_iter().enumerate() {
            plist.array_insert_item(Plist::new_string(&(key.into())), i as u32)?;
        }

        Ok(relays
            .into_iter()
            .map(|relay| {
                let plist = plist.clone();
                relay.query_mobilegestalt(plist)
            })
            .collect::<Result<Vec<_>, _>>()?)
    }

    /// Queries a specific IORegistry entry key for all devices in the group.
    ///
    /// # Arguments
    /// - `key`: The IORegistry entry key to query.
    ///
    /// # Returns
    /// A vector of `Plist` objects containing the queried information for each device.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the query fails for any device.
    ///
    /// # Note
    /// Currently, this function may panic for certain cases.
    pub fn query_ioregentry_key_all(
        &self,
        key: impl Into<String>,
    ) -> Result<Vec<Plist>, DeviceDiagnosticError> {
        self.device.check_all_connected::<DeviceDiagnosticError>()?;
        let relays = self.get_diagnostic_relaies()?;

        let key: String = key.into();

        Ok(relays
            .into_iter()
            .map(|relay| relay.query_ioregistry_entry(&key, ""))
            .collect::<Result<Vec<_>, _>>()?)
    }

    /// Queries diagnostics for all devices in the group.
    ///
    /// # Arguments
    /// - `r#type`: The type of diagnostics to query.
    ///
    /// # Returns
    /// A vector of `Plist` objects containing the diagnostics for each device.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the query fails for any device.
    pub fn query_diagnostics_all(
        &self,
        r#type: DiagnosticType,
    ) -> Result<Vec<Plist>, DeviceDiagnosticError> {
        self.device.check_all_connected::<DeviceDiagnosticError>()?;
        let relays = self.get_diagnostic_relaies()?;

        Ok(relays
            .into_iter()
            .map(|relay| relay.request_diagnostics(r#type.to_string()))
            .collect::<Result<Vec<_>, _>>()?)
    }

    /// Sends a sleep command to all devices in the group.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the command fails for any device.
    pub fn sleep_all(&self) -> Result<(), DeviceDiagnosticError> {
        self.device.check_all_connected::<DeviceDiagnosticError>()?;
        self.devices_power_action(DevicePowerAction::Sleep)
    }

    /// Sends a restart command to all devices in the group.
    ///
    /// # Arguments
    /// - `flag`: Defines the diagnostic behavior for the restart action.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the command fails for any device.
    pub fn restart_all(&self, flag: DiagnosticBehavior) -> Result<(), DeviceDiagnosticError> {
        self.device.check_all_connected::<DeviceDiagnosticError>()?;
        self.devices_power_action(DevicePowerAction::Restart(flag))
    }

    /// Sends a shutdown command to all devices in the group.
    ///
    /// # Arguments
    /// - `flag`: Defines the diagnostic behavior for the shutdown action.
    ///
    /// # Errors
    /// Returns `DeviceDiagnosticError` if the command fails for any device.
    pub fn shutdown_all(&self, flag: DiagnosticBehavior) -> Result<(), DeviceDiagnosticError> {
        self.device.check_all_connected::<DeviceDiagnosticError>()?;
        self.devices_power_action(DevicePowerAction::Shutdown(flag))
    }
}
