use crate::{device::DeviceClient, devices::SingleDevice};
use std::{fmt::Display, marker::PhantomData};
mod errors;

use errors::DeviceDiagnosticError;
use plist_plus::Plist;
use rusty_libimobiledevice::services::diagnostics_relay::DiagnosticsRelay;

const DIAGNOSTICS_RELAY_SERVICE: &str = "com.apple.mobile.diagnostics_relay";

#[allow(dead_code)]
// we should use it as a fallback
const DIAGNOSTICS_RELAY_SERVICE_OLD: &str = "com.apple.iosdiagnostics.relay";

#[derive(Debug)]
pub struct DeviceDiagnostic<T> {
    device: DeviceClient<T>,
    _phantom: PhantomData<T>,
}

#[derive(Debug)]
enum DevicePowerAction {
    Sleep,
    Shutdown(DiagnosticBehavior),
    Restart(DiagnosticBehavior),
}

#[derive(Debug)]
pub enum DiagnosticBehavior {
    /// wait until the diagnostic relay gets freed before execution
    WaitUntilDisconnected = 1 << 1, // Equivalent to 2
    ShowSuccessMessage = 1 << 2, // Equivalent to 4
    ShowFailureMessage = 1 << 3, // Equivalent to 8
}

pub enum IORegPlane {
    IODeviceTree,
    IOPower,
    IOService,
    None,
}

impl Display for IORegPlane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IORegPlane::None => write!(f, "None"),
            IORegPlane::IOPower => write!(f, "IOPower"),
            IORegPlane::IOService => write!(f, "IOService"),
            IORegPlane::IODeviceTree => write!(f, "IODeviceTree"),
        }
    }
}

impl<T> DeviceDiagnostic<T> {
    pub fn new(device: DeviceClient<T>) -> DeviceDiagnostic<T> {
        DeviceDiagnostic {
            device,
            _phantom: PhantomData::<T>,
        }
    }
}

impl DeviceDiagnostic<SingleDevice> {
    fn _get_diagnostic_relay(&self) -> Result<DiagnosticsRelay, DeviceDiagnosticError> {
        let device = self
            .device
            .get_device()
            .ok_or(DeviceDiagnosticError::DeviceNotFound)?;

        let mut lockdown = self.device.get_lockdown_client()?;

        let diagnostic_service = lockdown
            .start_service(DIAGNOSTICS_RELAY_SERVICE, true)
            .map_err(|e| DeviceDiagnosticError::ServiceError(e.to_string()))?;

        let relay = DiagnosticsRelay::new(device, diagnostic_service)
            .map_err(|e| DeviceDiagnosticError::RelayInitializationError(e.to_string()))?;

        Ok(relay)
    }
    fn _device_power_action(&self, action: DevicePowerAction) -> Result<(), DeviceDiagnosticError> {
        let relay = self._get_diagnostic_relay()?;
        match action {
            DevicePowerAction::Sleep => relay.sleep()?,
            DevicePowerAction::Restart(flag) => relay.restart(flag as core::ffi::c_uint)?,
            DevicePowerAction::Shutdown(flag) => relay.shutdown(flag as core::ffi::c_uint)?,
        }
        Ok(())
    }

    pub fn query_ioreg_plane(&self, plane: IORegPlane) -> Result<Plist, DeviceDiagnosticError> {
        let relay = self._get_diagnostic_relay()?;
        Ok(relay.query_ioregistry_plane(plane.to_string())?)
    }

    pub fn query_ioregentry_key(
        &self,
        key: impl Into<String>,
    ) -> Result<Plist, DeviceDiagnosticError> {
        let relay = self._get_diagnostic_relay()?;
        Ok(relay.query_ioregistry_entry(key, "")?)
    }

    pub fn query_mobilegestalt(
        &self,
        keys: Vec<impl Into<String>>,
    ) -> Result<Plist, DeviceDiagnosticError> {
        let relay = self._get_diagnostic_relay()?;
        let mut plist = Plist::new_array();

        for (i, key) in keys.into_iter().enumerate() {
            plist.array_insert_item(Plist::new_string(&(key.into())), i as u32)?;
        }
        Ok(relay.query_mobilegestalt(plist)?)
    }
    pub fn sleep(&self) -> Result<(), DeviceDiagnosticError> {
        self._device_power_action(DevicePowerAction::Sleep)
    }
    pub fn restart(&self, flag: DiagnosticBehavior) -> Result<(), DeviceDiagnosticError> {
        self._device_power_action(DevicePowerAction::Restart(flag))
    }
    pub fn shutdown(&self, flag: DiagnosticBehavior) -> Result<(), DeviceDiagnosticError> {
        self._device_power_action(DevicePowerAction::Shutdown(flag))
    }
}
