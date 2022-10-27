use auip::{
    storage::dynamic::{Addrs, Arp, IpFragment},
    Interface,
};
use auip_pkt::{layer2, layer3};
use auip_tap::TapTunDevice;
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

    let device = TapTunDevice::new_tap("tap0").unwrap();

    let mut addrs_storage = Addrs::default();

    addrs_storage.set_mac_addr(layer2::Address::parse("33:76:65:00:00:01").unwrap());

    let ipv4_addr = layer3::Address::Ipv4(layer3::ipv4::Address::parse("192.168.69.101").unwrap());
    let cidr = layer3::Cidr::new(ipv4_addr, 24);
    addrs_storage.add_ip_addr(cidr).unwrap();

    let arp_storage = Arp::default();

    let ip_fragment = IpFragment::new(5);

    let mut iface = Interface::new(device, addrs_storage, arp_storage, ip_fragment);

    loop {
        iface.device_mut().poll_read();
        if let Err(e) = iface.poll() {
            log::error!("{:?}", e);
        }
    }
}
