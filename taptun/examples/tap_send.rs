use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    socket.connect("192.168.69.101:8080")?;

    let buffer = [1u8; 2000];

    socket.send(&buffer)?;
    Ok(())
}
