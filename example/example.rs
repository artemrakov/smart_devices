use devices::{SmartSocket, SmartSocketState, SmartThermometer};
use house::room::Room;
use house::SmartHome;
use provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};

fn main() {
    let socket1 = SmartSocket::new(String::from("socket_1"), SmartSocketState::On);
    let socket2 = SmartSocket::new(String::from("socket_2"), SmartSocketState::Off);
    let thermo = SmartThermometer::new(String::from("thermo"),String::from("27.0"));

    let room1 = Room::new("Room 1".to_string(), vec!["socket_1".to_string(), "socket_2".to_string()]);
    let room2 = Room::new("Room 2".to_string(), vec!["thermo".to_string(), "socket_2".to_string()]);

    let mut house = SmartHome::new("House :)".to_string());
    house.add_room(room1);
    house.add_room(room2);

    let info_provider_1 = OwningDeviceInfoProvider { socket: socket1 };
    let report1 = house.create_report(&info_provider_1);

    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report2 = house.create_report(&info_provider_2);

    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
