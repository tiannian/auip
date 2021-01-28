pub struct Auip {}

impl Auip {
    pub fn new() -> Auip {
        Auip {}
    }

    pub async fn trigger() {
        // receive all device packet.
        // select interface and send to interface.
        // receive packet
    }

    pub fn alloc_udp_socket() -> UdpSocket {
        UdpSocket::new()
    }
}

pub struct UdpSocket {}

impl UdpSocket {
    pub fn new() -> UdpSocket {
        UdpSocket {}
    }
}
