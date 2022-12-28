pub mod room;

use std::collections::HashMap;

use room::Room;
use crate::provider::DeviceInfoProvider;

pub struct SmartHome {
    description: String,
    rooms: Vec<Room>,
}

impl SmartHome {
    pub fn new(description: String) -> Self {
        SmartHome {
            description,
            rooms: vec![],
        }
    }

    pub fn add_room(&mut self, room: Room) {
        let same_room = self.get_rooms().iter().find(|r| r.get_name() == room.get_name());

        if let None = same_room {
            self.rooms.push(room)
        }
    }

    fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn get_devices(&self, room: &str) -> &Vec<String> {
        let found_room = self.get_rooms().iter().find(|r| r.get_name() == room).unwrap();
        &found_room.get_devices()
    }

    pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
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
                room.get_devices()
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

        let mut result = vec![format!("Finding report of {}", &self.description)];

        // check if all required_devices are found
        device_report.drain().for_each(|(_, mut value)| {
            if value.len() == 0 {
                panic!("Device not found")
            }

            result.append(&mut value)
        });

        result.join("\n")
    }
}
