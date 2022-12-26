use std::collections::HashMap;

use crate::devices::SmartSocketState;
use devices::{Devices, SmartSocket, SmartThermometer};

pub mod devices;

struct Room {
    devices: Vec<String>,
    name: String,
}

impl Room {
    fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    fn add_device(mut self, device: String) {
        // ????
        let same_device = self.devices.iter().find(|&dev| *dev == device);

        if let None = same_device {
            self.devices.push(device)
        }
    }

    fn get_devices(&self) -> &[String] {
        self.devices.as_ref()
    }
}

struct SmartHome {
    description: String,
    rooms: Vec<Room>,
}

impl SmartHome {
    fn new() -> Self {
        SmartHome {
            description: String::from("Smart house"),
            rooms: vec![],
        }
    }

    fn add_room(mut self, room: Room) {
        let same_room = self.get_rooms().iter().find(|r| r.name == room.name);

        if let None = same_room {
            self.rooms.push(room)
        }
    }

    fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn get_devices(&self, room: &str) -> &Vec<String> {
        let found_room = self.get_rooms().iter().find(|r| r.name == room).unwrap();
        &found_room.devices
    }

    fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
        let tuples: Vec<(&str, Vec<String>)> = provider
            .required_devices()
            .iter()
            .map(|device| (device.get_name(), Vec::new()))
            .collect();
        let mut device_report: HashMap<&str, Vec<String>> = tuples.into_iter().collect();

        let room_devices: Vec<(&Room, &str)> = self
            .get_rooms()
            .iter()
            .flat_map(|room| {
                room.devices
                    .iter()
                    .map(|device| (room, device as &str))
                    .collect::<Vec<(&Room, &str)>>()
            })
            .filter(|(_, device)| device_report.contains_key(device))
            .collect();

        room_devices.iter().for_each(|(room, device)| {
            let report = device_report.get_mut(device).unwrap();
            report.push(provider.get_info(room.get_name(), device))
        });

        // check if all required_devices are found
        device_report.values().for_each(|report| {
            if report.len() == 0 {
                panic!("Device not found")
            }
        });

        format!("Finding report of {}", &self.description)
    }
}

trait DeviceInfoProvider {
    fn get_info(&self, room: &str, device: &str) -> String;
    fn required_devices(&self) -> Vec<&dyn Devices>;
}

struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
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
        format!("Room: {}, Device: {}", room, dev.report())
    } else {
        String::from("")
    }
}

fn main() {
    let socket1 = SmartSocket {
        name: String::from("Socket 1"),
        state: SmartSocketState::On,
    };
    let socket2 = SmartSocket {
        name: String::from("Socket 2"),
        state: SmartSocketState::Off,
    };
    let thermo = SmartThermometer {
        name: String::from("Thermo"),
        temperature: String::from("27.0"),
    };

    let house = SmartHome::new();

    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
