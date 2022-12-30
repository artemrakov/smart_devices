pub trait Device {
    fn get_name(&self) -> &str;
    fn report(&self) -> String;
}

#[derive(Debug)]
pub enum SmartSocketState {
    On,
    Off,
}

pub struct SmartSocket {
    state: SmartSocketState,
    name: String,
}

impl SmartSocket {
    pub fn new(name: String, state: SmartSocketState) -> Self {
        SmartSocket { name, state }
    }
}

pub struct SmartThermometer {
    name: String,
    temperature: String,
}

impl SmartThermometer {
    pub fn new(name: String, temperature: String) -> Self {
        SmartThermometer { name, temperature }
    }
}

impl Device for SmartSocket {
    fn report(&self) -> String {
        format!("Socket: {} and state is {:?}", self.name, self.state)
    }

    fn get_name(&self) -> &str {
        self.name.as_ref()
    }
}

impl Device for SmartThermometer {
    fn report(&self) -> String {
        format!(
            "Thermometer: {} and temperature is {}",
            self.name, self.temperature
        )
    }

    fn get_name(&self) -> &str {
        self.name.as_ref()
    }
}
