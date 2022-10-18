use super::arp;
// use super::ipv4;
// use super::ipv6;
use crate::error::*;
use crate::ip;
use crate::mac;

#[derive(Debug, Clone)]
pub enum Packet<T: AsRef<[u8]>> {
    // IPv4(ipv4::Packet<T>),
    ARP(arp::Packet<T>),
    IPv6,
}

impl<T: AsRef<[u8]>> Packet<T> {
    pub fn from_layer2_pkt(pkt: mac::Packet<&[u8]>) -> Result<ip::Packet<&[u8]>> {
        // let protocol = pkt.protocol();
        let buffer = pkt.payload()?;
        match pkt.protocol() {
            mac::Protocol::ARP => {
                let arp = arp::Packet::new_checked(buffer)?;
                Ok(Packet::ARP(arp))
            }
            _ => Ok(Packet::IPv6),
        }
    }
}
