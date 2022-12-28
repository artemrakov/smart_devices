pub mod room;

use std::collections::HashMap;

use crate::provider::DeviceInfoProvider;
use room::Room;

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
        let same_room = self
            .get_rooms()
            .iter()
            .find(|r| r.get_name() == room.get_name());

        if let None = same_room {
            self.rooms.push(room)
        }
    }

    fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }

    fn get_devices(&self, room: &str) -> &Vec<String> {
        let found_room = self
            .get_rooms()
            .iter()
            .find(|r| r.get_name() == room)
            .unwrap();
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{devices::{SmartSocket, SmartSocketState}, provider::OwningDeviceInfoProvider};

    #[test]
    fn can_add_room() {
        let mut house = SmartHome::new(String::from("new house"));
        let room = Room::new(String::from("room"), Vec::new());

        house.add_room(room);

        let room_names: Vec<&str> = house.get_rooms().iter().map(|r| r.get_name()).collect();
        assert!(room_names.contains(&"room"));
    }

    #[test]
    fn cannot_add_room_with_same_name() {
        let mut house = SmartHome::new(String::from("new house"));
        let room1 = Room::new(String::from("room"), Vec::new());
        let room2 = Room::new(String::from("room"), Vec::new());

        house.add_room(room1);
        house.add_room(room2);

        assert_eq!(house.get_rooms().len(), 1);
    }

    #[test]
    fn creates_report() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let room1 = Room::new(
            "Room 1".to_string(),
            vec!["socket_1".to_string(), "socket_2".to_string()],
        );
        let room2 = Room::new(
            "Room 2".to_string(),
            vec!["thermo".to_string(), "socket_2".to_string()],
        );
        let mut house = SmartHome::new("House :)".to_string());
        house.add_room(room1);
        house.add_room(room2);
        let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

        let report1 = house.create_report(&info_provider_1);

        assert!(report1.contains("Room: Room 1, Device Socket: socket_1 and state is On"));
        assert!(!report1.contains("Room: Room 1, Device Socket: socket_2"));
        assert!(!report1.contains("Room: Room 2, Device Socket: socket_2"));
        assert!(!report1.contains("Room: Room 2, Device Thermometer: thermo"));
    }

    #[test]
    #[should_panic(expected="Device not found")]
    fn device_not_found() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let room1 = Room::new(
            "Room 1".to_string(),
            vec![],
        );
        let mut house = SmartHome::new("House :)".to_string());
        house.add_room(room1);
        let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

        house.create_report(&info_provider_1);
    }
}
