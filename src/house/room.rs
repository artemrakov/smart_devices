use std::collections::HashSet;

#[derive(Debug)]
pub struct Room {
    devices: HashSet<String>,
    name: String,
}

impl Room {
    pub fn new(name: String, devices: HashSet<String>) -> Self {
        Room { devices, name }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn add_device(&mut self, device: String) -> Option<bool> {
        if !self.devices.contains(&device) {
            self.devices.insert(device);

            return Some(true);
        }

        None
    }

    pub fn get_devices(&self) -> &HashSet<String> {
        &self.devices
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.get_name() == other.get_name()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_add_devise() {
        let mut room = Room::new(String::from("room"), HashSet::new());
        let device_name = String::from("device");

        room.add_device(device_name);

        assert!(room.get_devices().contains("device"));
    }

    #[test]
    fn cannot_add_same_device() {
        let mut room = Room::new(String::from("room"), HashSet::new());

        room.add_device(String::from("device"));
        room.add_device(String::from("device"));

        assert_eq!(room.get_devices().len(), 1);
    }
}
