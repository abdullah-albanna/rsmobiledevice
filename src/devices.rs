use rusty_libimobiledevice::idevice::Device;

#[derive(Debug)]
pub struct SingleDevice();
#[derive(Debug)]
pub struct DeviceGroup();

#[derive(Debug)]
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
