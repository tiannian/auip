use auip::Interface;
use auip_tap::{open_tap_device, TapTunDevice};
use std::process::Command;

fn main() {
    env_logger::init();
    let mut command = Command::new("ip")
        .arg("addr")
        .arg("add")
        .arg("192.168.69.100/24")
        .arg("dev")
        .arg("tap0")
        .spawn()
        .unwrap();
    let _ = command.wait().unwrap();
    let mut command = Command::new("ip")
        .arg("link")
        .arg("set")
        .arg("tap0")
        .arg("up")
        .spawn()
        .unwrap();
    let _ = command.wait().unwrap();

    let file = open_tap_device("tap0").unwrap();
    let device = TapTunDevice::new(file);

    // let iface = Interface::new(device, addrs_storage, arp_storage)
}
