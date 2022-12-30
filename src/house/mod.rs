pub mod room;

use std::collections::{HashMap, HashSet};

use crate::provider::DeviceInfoProvider;
use room::Room;

pub struct SmartHome {
    description: String,
    rooms: HashMap<String, Room>,
}

impl SmartHome {
    pub fn new(description: String) -> Self {
        SmartHome {
            description,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) {
        if !self.rooms.contains_key(room.get_name()) {
            self.rooms.insert(String::from(room.get_name()), room);
        }
    }

    fn get_rooms(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    fn get_devices(&self, room: &str) -> &HashSet<String> {
        self.rooms.get(room).unwrap().get_devices()
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
            .flat_map(|(_, room)| {
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
    use crate::{
        device::{SmartSocket, SmartSocketState},
        provider::OwningDeviceInfoProvider,
    };

    #[test]
    fn can_add_room() {
        let mut house = SmartHome::new(String::from("new house"));
        let room = Room::new(String::from("room"), HashSet::new());

        house.add_room(room);

        assert!(house.get_rooms().contains_key("room"));
    }

    #[test]
    fn cannot_add_room_with_same_name() {
        let mut house = SmartHome::new(String::from("new house"));
        let room1 = Room::new(String::from("room"), HashSet::new());
        let room2 = Room::new(String::from("room"), HashSet::new());

        house.add_room(room1);
        house.add_room(room2);

        assert_eq!(house.get_rooms().len(), 1);
    }

    #[test]
    fn creates_report() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let room1 = Room::new(
            "Room 1".to_string(),
            HashSet::from_iter(vec!["socket_1".to_string(), "socket_2".to_string()]),
        );
        let room2 = Room::new(
            "Room 2".to_string(),
            HashSet::from_iter(vec!["thermo".to_string(), "socket_2".to_string()]),
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
    #[should_panic(expected = "Device not found")]
    fn device_not_found() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let room1 = Room::new("Room 1".to_string(), HashSet::new());
        let mut house = SmartHome::new("House :)".to_string());
        house.add_room(room1);
        let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

        house.create_report(&info_provider_1);
    }
}
