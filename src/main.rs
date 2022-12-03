fn main() {
    let socket = SmartSocket {
        description: String::from("tst"),
        state: SmartSocketState::On,
    };

    println!("{:?}", socket);

    let socket2 = socket.turn_off();
    println!("{:?}", socket2);

    socket2.turn_on();
}

#[derive(Debug)]
enum SmartSocketState {
    On,
    Off,
}

#[derive(Debug)]
struct SmartSocket {
    description: String,
    state: SmartSocketState,
}

impl SmartSocket {
    fn turn_on(self) -> Self {
        SmartSocket {
            state: SmartSocketState::On,
            ..self
        }
    }

    fn turn_off(self) -> Self {
        SmartSocket {
            state: SmartSocketState::Off,
            ..self
        }
    }

    fn _power_consumption(&self) -> String {
        todo!()
    }
}

struct Thermometer {
    temperature: String,
}

impl Thermometer {
    fn temperature(&self) -> String {
        todo!()
    }
}
