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
