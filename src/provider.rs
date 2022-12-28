use std::collections::HashMap;

use crate::devices::{Devices, SmartSocket, SmartThermometer};

pub trait DeviceInfoProvider {
    fn get_info(&self, room: &str, device: &str) -> String;
    fn required_devices(&self) -> Vec<&dyn Devices>;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn required_devices(&self) -> Vec<&dyn Devices> {
        vec![&self.socket]
    }

    fn get_info(&self, room: &str, device: &str) -> String {
        let devices: HashMap<&str, &dyn Devices> =
            HashMap::from([(self.socket.get_name(), &self.socket as &dyn Devices)]);

        extract_info(room, device, devices)
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_info(&self, room: &str, device: &str) -> String {
        let devices: HashMap<&str, &dyn Devices> = HashMap::from([
            (self.socket.get_name(), self.socket as &dyn Devices),
            (self.thermo.get_name(), self.thermo as &dyn Devices),
        ]);

        extract_info(room, device, devices)
    }

    fn required_devices(&self) -> Vec<&dyn Devices> {
        vec![self.socket, self.thermo]
    }
}

fn extract_info(room: &str, device: &str, devices: HashMap<&str, &dyn Devices>) -> String {
    let maybe_device = devices.get(device);

    if let Some(dev) = maybe_device {
        format!("Room: {}, Device {}", room, dev.report())
    } else {
        String::from("")
    }
}
