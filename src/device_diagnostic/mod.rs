use crate::{device::DeviceClient, devices_collection::SingleDevice};
use enums::{DevicePowerAction, DiagnosticBehavior, DiagnosticType, IORegPlane};
use errors::DeviceDiagnosticError;
use plist_plus::Plist;
use rusty_libimobiledevice::services::diagnostics_relay::DiagnosticsRelay;
use std::marker::PhantomData;

pub mod enums;
pub mod errors;

const DIAGNOSTICS_RELAY_SERVICE: &str = "com.apple.mobile.diagnostics_relay";

#[allow(dead_code)]
// we should use it as a fallback
const DIAGNOSTICS_RELAY_SERVICE_OLD: &str = "com.apple.iosdiagnostics.relay";

#[derive(Debug)]
pub struct DeviceDiagnostic<T> {
    device: DeviceClient<T>,
    _phantom: PhantomData<T>,
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
        self.device.check_connected::<DeviceDiagnosticError>()?;
        let relay = self._get_diagnostic_relay()?;
        Ok(relay.query_ioregistry_plane(plane.to_string())?)
    }

    pub fn query_ioregentry_key(
        &self,
        key: impl Into<String>,
    ) -> Result<Plist, DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        let relay = self._get_diagnostic_relay()?;
        Ok(relay.query_ioregistry_entry(key, "")?)
    }

    pub fn query_mobilegestalt(
        &self,
        keys: Vec<impl Into<String>>,
    ) -> Result<Plist, DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        let relay = self._get_diagnostic_relay()?;
        let mut plist = Plist::new_array();

        for (i, key) in keys.into_iter().enumerate() {
            plist.array_insert_item(Plist::new_string(&(key.into())), i as u32)?;
        }
        Ok(relay.query_mobilegestalt(plist)?)
    }

    pub fn query_diagnostics(
        &self,
        r#type: DiagnosticType,
    ) -> Result<Plist, DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        let relay = self._get_diagnostic_relay()?;
        Ok(relay.request_diagnostics(r#type.to_string())?)
    }
    pub fn sleep(&self) -> Result<(), DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        self._device_power_action(DevicePowerAction::Sleep)
    }
    pub fn restart(&self, flag: DiagnosticBehavior) -> Result<(), DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        self._device_power_action(DevicePowerAction::Restart(flag))
    }
    pub fn shutdown(&self, flag: DiagnosticBehavior) -> Result<(), DeviceDiagnosticError> {
        self.device.check_connected::<DeviceDiagnosticError>()?;
        self._device_power_action(DevicePowerAction::Shutdown(flag))
    }
}
