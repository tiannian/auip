// use tokio::fs::File;
use auip::phy::Device;
use auip_tap::open_tap_device;
use auip_tap::TapDevice;
use tokio::process::Command;

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
    // let pkt = device.receive().await;
}
