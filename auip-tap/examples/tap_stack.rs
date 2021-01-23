// use tokio::fs::File;
use auip_tap::open_tap_device;
use tokio::io::AsyncReadExt;
use tokio::process::Command;

#[tokio::main]
async fn main() {
    let mut file = open_tap_device("tap0").await.unwrap();
    let mut buffer = [0u8; 128];
    file.read(&mut buffer).await.unwrap();
    let mut command = Command::new("ip")
        .arg("addr add 192.168.69.100/24 dev tap0")
        .spawn()
        .unwrap();
    let status = child.wait().await?;
}
