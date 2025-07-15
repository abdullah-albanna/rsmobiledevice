//! # rsmobiledevice Library
//!
//! `rsmobiledevice` is a Rust library designed to interact with iOS devices using
//! [rusty_libimobiledevice](https://github.com/jkcoxson/rusty_libimobiledevice) through a Rust abstraction layer.
//! This library provides functionality for managing devices, accessing diagnostics,
//! fetching device information, handling installations, and more.
//!
//! ## Modules
//! - `device`: Core device abstractions and utilities.
//! - `device_diagnostic`: Tools for retrieving and analyzing device diagnostics.
//! - `device_info`: Functionality to fetch detailed information about devices.
//! - `device_installer`: Provides support for installing applications on devices, supporting both ipcc and ipa.
//! - `device_syslog`: Access to the system logs of devices.
//!
//! ## Features
//! - Recursive search functionality in `Plist` structures via the `RecursiveFind` trait to look for any key at any part.
//! - Modular design for ease of integration.
//! - Comprehensive error handling for robust applications.

use plist_plus::{Plist, PlistType};

pub mod device;
pub mod device_afcclient;
pub mod device_diagnostic;
pub mod device_info;
pub mod device_installer;
pub mod device_syslog;
pub mod devices_collection;
pub mod errors;

/// Trait providing recursive search functionality for `Plist` structures.
///
/// This trait allows traversing `Plist` objects to locate a value associated with a specific key.
/// It works recursively on nested dictionaries.
pub trait RecursiveFind {
    /// Recursively searches for the given key within the `Plist` structure.
    fn rfind(&self, key: &str) -> Option<String>;
}

impl RecursiveFind for Plist {
    /// Implementation of the recursive search functionality.
    ///
    /// This method checks if the current `Plist` contains the desired key. If the key is not
    /// present at the current level, the method will traverse nested dictionaries to locate it.
    fn rfind(&self, key: &str) -> Option<String> {
        for part in self.clone() {
            // If the key matches, return its display value without quotes.
            if part.key.map_or(false, |s| s == key) {
                return part
                    .plist
                    .get_display_value()
                    .map(|s| s.replace("\"", ""))
                    .ok();
            }

            // If the key is not found, recursively search in nested dictionaries.
            if let PlistType::Dictionary = part.plist.plist_type {
                if let Some(value) = part.plist.rfind(key) {
                    return Some(value);
                }
            }
        }
        None
    }
}
