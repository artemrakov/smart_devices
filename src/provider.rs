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

#[cfg(test)]
mod test {
    use super::*;
    use crate::devices::SmartSocketState;

    #[test]
    fn create_report_for_owning_device() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let info_provider = OwningDeviceInfoProvider { socket: socket1 };

        let report = info_provider.get_info("room", "socket_1");
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

        let info1 = info_provider.get_info("room", "socket_1");
        let info2 = info_provider.get_info("room", "thermo");

        assert!(info1.contains("Room: room, Device Socket: socket_1"));
        assert!(info2.contains("Room: room, Device Thermometer: thermo"));
    }

    #[test]
    fn do_not_return_report_if_device_is_not_found() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let info_provider = OwningDeviceInfoProvider { socket: socket1 };

        let info = info_provider.get_info("room", "not_found");
        assert_eq!(info, "");
    }
}
