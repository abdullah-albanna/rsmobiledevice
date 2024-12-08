use rusty_libimobiledevice::idevice::Device;

#[derive(Debug, Clone)]
pub struct SingleDevice();
#[derive(Debug, Clone)]
pub struct DeviceGroup();

#[derive(Debug, Clone)]
pub enum Devices {
    Single(Device),
    Multiple(Vec<Device>),
}

impl Devices {
    pub fn get_device(&self) -> Option<&Device> {
        if let Devices::Single(device) = self {
            Some(device)
        } else {
            None
        }
    }

    pub fn get_devices(&self) -> Option<&Vec<Device>> {
        if let Devices::Multiple(devices) = self {
            Some(devices)
        } else {
            None
        }
    }
}
