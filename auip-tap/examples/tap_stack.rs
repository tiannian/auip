// use tokio::fs::File;
use auip::phy::Driver;
use auip_tap::open_tap_device;
use auip_tap::TapDevice;
use tokio::process::Command;
use auip_pkt::prelude::PayloadMut;

#[tokio::main]
async fn main() {
    let file = open_tap_device("tap0").await.unwrap();
    let mut command = Command::new("ip")
        .arg("addr")
        .arg("add")
        .arg("192.168.69.100/24")
        .arg("dev")
        .arg("tap0")
        .spawn()
        .unwrap();
    let _ = command.wait().await.unwrap();
    let mut command = Command::new("ip")
        .arg("link")
        .arg("set")
        .arg("tap0")
        .arg("up")
        .spawn()
        .unwrap();
    let _ = command.wait().await.unwrap();
    let mut device = TapDevice::new(file);
    let buf = device.receive().await;
    let mut layer2_pkt = auip_pkt::mac::ethernet::Packet::new_checked(buf).unwrap();
    println!("{}", layer2_pkt);
    let layer2_payload = layer2_pkt.payload_mut().unwrap();
    let layer3_pkt = auip_pkt::ip::arp::Packet::new_checked(layer2_payload).unwrap();
    println!("{}", layer3_pkt);
}
