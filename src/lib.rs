use plist_plus::{Plist, PlistType};

pub mod device;
pub mod device_diagnostic;
pub mod device_info;
pub mod device_installer;
pub mod device_syslog;
pub mod errors;

pub mod devices_collection;

pub trait RecursiveFind {
    fn rfind(&self, key: &str) -> Option<String>;
}

impl RecursiveFind for Plist {
    fn rfind(&self, key: &str) -> Option<String> {
        for part in self.clone() {
            // if we find the key we immediately return it
            if part.key.map_or(false, |s| s == key) {
                return part
                    .plist
                    .get_display_value()
                    // removes the "" in the display value
                    .map(|s| s.replace("\"", ""))
                    .ok();
            }

            // if we did not find the key, we check if it's a dictionary first, if so we recursivly
            // call rfind until we are not in a dictionary any more
            if let PlistType::Dictionary = part.plist.plist_type {
                if let Some(value) = part.plist.rfind(key) {
                    return Some(value);
                }
            }
        }
        None
    }
}
