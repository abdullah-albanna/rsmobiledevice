//! Provides abstractions for managing single or multiple iOS devices.
//!
//! This module defines the `Devices` enum, which can represent either a single device or
//! multiple devices.

use rusty_libimobiledevice::idevice::Device;

/// Marker type representing a single device.
///
/// This struct is primarily used as a type parameter in generic contexts
/// to indicate operations involving a single device.
#[derive(Debug, Clone, PartialEq)]
pub struct SingleDevice();

/// Marker type representing a group of devices.
///
/// This struct is primarily used as a type parameter in generic contexts
/// to indicate operations involving multiple devices.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceGroup();

/// Enum representing either a single device or a group of devices.
///
/// This abstraction allows handling both individual and multiple devices
/// with a unified API.
#[derive(Debug, Clone, PartialEq)]
pub enum Devices {
    /// A single device.
    Single(Device),
    /// Multiple devices.
    Multiple(Vec<Device>),
}

impl Devices {
    /// Retrieves the single device if this instance represents a single device.
    ///
    /// # Returns
    /// - `Some(&Device)` if this is a `Devices::Single` variant.
    /// - `None` otherwise.
    ///
    pub fn get_device(&self) -> Option<&Device> {
        if let Devices::Single(device) = self {
            Some(device)
        } else {
            None
        }
    }

    /// Retrieves the list of devices if this instance represents multiple devices.
    ///
    /// # Returns
    /// - `Some(&Vec<Device>)` if this is a `Devices::Multiple` variant.
    /// - `None` otherwise.
    ///
    pub fn get_devices(&self) -> Option<&Vec<Device>> {
        if let Devices::Multiple(devices) = self {
            Some(devices)
        } else {
            None
        }
    }
}
