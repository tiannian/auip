use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:34254")?;
    socket.connect("192.168.69.101:8080")?;
    socket.send(&[0, 1, 2])?;
    Ok(())
}
