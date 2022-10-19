use super::arp;
use super::ipv4;

#[derive(Debug, Clone)]
pub enum Packet<T> {
    IPv4(ipv4::Packet<T>),
    ARP(arp::Packet<T>),
}

impl<T: AsRef<[u8]>> Packet<T> {
    pub fn parse_ip(t: T) {

    }
}

