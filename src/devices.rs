use rusty_libimobiledevice::idevice::Device;

pub struct SingleDevice();
pub struct DeviceGroup();

#[derive(Debug, Clone)]
pub enum Devices {
    Single(Device),
    Multiple(Vec<Device>),
}

impl Devices {
    pub fn get_device(&self) -> Option<Device> {
        if let Devices::Single(device) = self {
            Some(device.clone())
        } else {
            None
        }
    }

    pub fn get_devices(&self) -> Option<Vec<Device>> {
        if let Devices::Multiple(devices) = self {
            Some(devices.clone())
        } else {
            None
        }
    }
}
