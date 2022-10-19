use crate::{Error, Result};

use super::{arp, ipv4};

#[derive(Debug, Clone)]
pub enum IpPacket<T> {
    IPv4(ipv4::Packet<T>),
    Ipv6,
}

impl<T: AsRef<[u8]>> IpPacket<T> {
    pub fn parse(t: T) -> Result<Self> {
        let data = t.as_ref();
        if data[ipv4::field::VER_IHL] >> 4 == 4 {
            let pkt = ipv4::Packet::new_checked(t)?;
            Ok(IpPacket::IPv4(pkt))
        } else {
            Err(Error::UnknownIpVersionNumber)
        }
    }
}

pub enum Packet<T> {
    Arp(arp::Packet<T>),
    IPv4(ipv4::Packet<T>),
    Ipv6,
}
