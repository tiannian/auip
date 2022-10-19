use super::arp;
use super::ipv4;

#[derive(Debug, Clone)]
pub enum Packet<T> {
    IPv4(ipv4::Packet<T>),
    ARP(arp::Packet<T>),
}
