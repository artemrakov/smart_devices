pub trait Devices {
    fn get_name(&self) -> &str;
    fn report(&self) -> String;
}

#[derive(Debug)]
pub enum SmartSocketState {
    On,
    Off,
}

pub struct SmartSocket {
    pub state: SmartSocketState,
    pub name: String,
}

impl SmartSocket {
    fn _turn_on(self) -> Self {
        SmartSocket {
            state: SmartSocketState::On,
            ..self
        }
    }

    fn _turn_off(self) -> Self {
        SmartSocket {
            state: SmartSocketState::Off,
            ..self
        }
    }
}

pub struct SmartThermometer {
    pub name: String,
    pub temperature: String,
}

impl Devices for SmartSocket {
    fn report(&self) -> String {
        format!("Socket: {} and state is {:?}", self.name, self.state)
    }

    fn get_name(&self) -> &str {
        self.name.as_ref()
    }
}

impl Devices for SmartThermometer {
    fn report(&self) -> String {
        format!("Thermometer: {} and temperature is {}", self.name, self.temperature)
    }

    fn get_name(&self) -> &str {
        self.name.as_ref()
    }
}
