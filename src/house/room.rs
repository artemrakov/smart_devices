#[derive(Debug)]
pub struct Room {
    devices: Vec<String>,
    name: String,
}

impl Room {
    pub fn new(name: String, devices: Vec<String>) -> Self {
        Room { devices, name }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn add_device(&mut self, device: String) {
        let same_device = self.devices.iter().find(|&dev| *dev == device);

        if let None = same_device {
            self.devices.push(device)
        }
    }

    pub fn get_devices(&self) -> &Vec<String> {
        self.devices.as_ref()
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
        let mut room = Room::new(String::from("room"), Vec::new());
        let device_name = String::from("device");

        room.add_device(device_name);

        assert_eq!(room.get_devices(), &vec![String::from("device")]);
    }

    #[test]
    fn cannot_add_same_device() {
        let mut room = Room::new(String::from("room"), Vec::new());

        room.add_device(String::from("device"));
        room.add_device(String::from("device"));

        assert_eq!(room.get_devices().len(), 1);
    }
}
