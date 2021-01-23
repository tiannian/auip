// use tokio::fs::File;
use auip_tap::open_tap_device;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let mut file = open_tap_device("tap0").await.unwrap();
    let mut command = Command::new("ip")
        .arg("addr")
        .arg("add")
        .arg("192.168.69.100/24")
        .arg("dev")
        .arg("tap0")
        .spawn()
        .unwrap();
    let status = command.wait().await.unwrap();
    println!("Set ip {}", status);
    let mut buffer = [0u8; 128];
    file.read(&mut buffer).await.unwrap();
}
