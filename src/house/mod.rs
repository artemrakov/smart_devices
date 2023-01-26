pub mod room;

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Error as FmtError, Formatter},
};

use crate::provider::DeviceInfoProvider;
use room::Room;

pub struct SmartHome {
    description: String,
    rooms: HashMap<String, Room>,
}

#[derive(Debug, Clone)]
pub enum ReportError {
    NoInfoProvided(String),
    NotFoundDevice(String),
}

impl Error for ReportError {}

impl Display for ReportError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::NoInfoProvided(device) => {
                write!(f, "Do not have info for this device: {}", device)
            }
            Self::NotFoundDevice(device) => write!(f, "Cannot find this device: {}", device),
        }
    }
}

impl SmartHome {
    pub fn new(description: String) -> Self {
        SmartHome {
            description,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) -> Option<bool> {
        if !self.rooms.contains_key(room.get_name()) {
            self.rooms.insert(String::from(room.get_name()), room);
            return Some(true);
        }

        None
    }

    fn get_rooms(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> Result<String, ReportError> {
        let required_devices: Vec<&str> = provider
            .required_devices()
            .iter()
            .map(|device| device.get_name())
            .collect();

        let mut device_report: HashMap<&str, Vec<String>> = required_devices
            .clone()
            .into_iter()
            .map(|device| (device, vec![]))
            .collect();

        for room in self.get_rooms().values() {
            for device in room.get_devices() {
                if required_devices.contains(&device.as_str()) {
                    let info = provider.get_info(room.get_name(), device);

                    if info.is_none() {
                        return Err(ReportError::NoInfoProvided(device.to_string()));
                    }

                    let report = device_report.get_mut(&device.as_str()).unwrap();
                    report.push(info.unwrap())
                }
            }
        }

        let mut result = vec![format!("Finding report of {}", &self.description)];
        for (device, mut value) in device_report.drain() {
            if value.is_empty() {
                return Err(ReportError::NotFoundDevice(device.to_string()));
            }

            result.append(&mut value)
        }

        Ok(result.join("\n"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        device::{SmartSocket, SmartSocketState},
        provider::OwningDeviceInfoProvider,
    };
    use std::collections::HashSet;

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

        let report1 = house.create_report(&info_provider_1).unwrap();

        assert!(report1.contains("Room: Room 1, Device Socket: socket_1 and state is On"));
        assert!(!report1.contains("Room: Room 1, Device Socket: socket_2"));
        assert!(!report1.contains("Room: Room 2, Device Socket: socket_2"));
        assert!(!report1.contains("Room: Room 2, Device Thermometer: thermo"));
    }

    #[test]
    fn device_not_found() {
        let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
        let room1 = Room::new("Room 1".to_string(), HashSet::new());
        let mut house = SmartHome::new("House :)".to_string());
        let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };

        house.add_room(room1);
        let report = house.create_report(&info_provider_1);

        assert!(report.is_err());
    }
}
