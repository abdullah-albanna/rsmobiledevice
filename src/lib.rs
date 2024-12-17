use plist_plus::{Plist, PlistType};

pub mod device;
pub mod device_diagnostic;
pub mod device_info;
pub mod device_installer;
pub mod device_syslog;
pub mod errors;

mod devices_collection;

pub trait RecursiveFind {
    fn rfind(&self, key: &str) -> Option<String>;
}

impl RecursiveFind for Plist {
    fn rfind(&self, key: &str) -> Option<String> {
        for part in self.clone() {
            if part.key.unwrap_or("unknown".into()) == key {
                return part
                    .plist
                    .get_display_value()
                    // removes the "" in the display value
                    .map(|s| s.replace("\"", ""))
                    .ok();
            }

            if let PlistType::Dictionary = part.plist.plist_type {
                if let Some(value) = part.plist.rfind(key) {
                    return Some(value);
                }
            }
        }
        None
    }
}
