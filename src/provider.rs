use std::collections::HashMap;

use crate::device::{Device, SmartSocket, SmartThermometer};

pub trait DeviceInfoProvider {
    fn get_info(&self, room: &str, device: &str) -> Option<String>;
    fn required_devices(&self) -> Vec<&dyn Device>;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn required_devices(&self) -> Vec<&dyn Device> {
        vec![&self.socket]
    }

    fn get_info(&self, room: &str, device: &str) -> Option<String> {
        let devices: HashMap<&str, &dyn Device> =
            HashMap::from([(self.socket.get_name(), &self.socket as &dyn Device)]);

        extract_info(room, device, devices)
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_info(&self, room: &str, device: &str) -> Option<String> {
        let devices: HashMap<&str, &dyn Device> = HashMap::from([
            (self.socket.get_name(), self.socket as &dyn Device),
            (self.thermo.get_name(), self.thermo as &dyn Device),
        ]);

        extract_info(room, device, devices)
    }

    fn required_devices(&self) -> Vec<&dyn Device> {
        vec![self.socket, self.thermo]
    }
}

fn extract_info(room: &str, device: &str, devices: HashMap<&str, &dyn Device>) -> Option<String> {
    let dev = devices.get(device)?;

    Some(format!("Room: {}, Device {}", room, dev.report()))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::device::SmartSocketState;

    #[test]
    fn create_report_for_owning_device() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let info_provider = OwningDeviceInfoProvider { socket: socket1 };

        let report = info_provider.get_info("room", "socket_1").unwrap();
        assert!(report.contains("Room: room, Device Socket: socket_1"));
    }

    #[test]
    fn create_report_for_borrowing_device() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let thermo = SmartThermometer::new(String::from("thermo"), String::from("27.6"));
        let info_provider = BorrowingDeviceInfoProvider {
            socket: &socket1,
            thermo: &thermo,
        };

        let info1 = info_provider.get_info("room", "socket_1").unwrap();
        let info2 = info_provider.get_info("room", "thermo").unwrap();

        assert!(info1.contains("Room: room, Device Socket: socket_1"));
        assert!(info2.contains("Room: room, Device Thermometer: thermo"));
    }

    #[test]
    fn do_not_return_report_if_device_is_not_found() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let info_provider = OwningDeviceInfoProvider { socket: socket1 };

        let info = info_provider.get_info("room", "not_found");
        assert!(info.is_none());
    }
}
