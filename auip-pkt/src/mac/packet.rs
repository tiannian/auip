use super::ethernet;
use super::{Address, Protocol};
use crate::error::*;
use core::fmt::{self, Display};
use core::format_args;
// use crate::ip;

#[derive(Debug, Clone)]
pub enum Packet<T: AsRef<[u8]>> {
    EthernetII(ethernet::Packet<T>),
}

impl<T: AsRef<[u8]>> Display for Packet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::EthernetII(pkt) => f.write_fmt(format_args!("EthernetII:\n {}", pkt)),
        }
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Packet<T> {
    fn as_ref(&self) -> &[u8] {
        match self {
            Packet::EthernetII(pkt) => pkt.as_ref(),
        }
    }
}

impl<T: AsRef<[u8]>> Packet<T> {
    pub fn protocol(&self) -> Protocol {
        match self {
            Packet::EthernetII(pkt) => pkt.protocol(),
        }
    }

    pub fn into_inner(self) -> T {
        match self {
            Packet::EthernetII(pkt) => pkt.into_inner(),
        }
    }

    pub fn dest_addr(&self) -> Address {
        match self {
            Packet::EthernetII(pkt) => pkt.dest_addr(),
        }
    }

    pub fn src_addr(&self) -> Address {
        match self {
            Packet::EthernetII(pkt) => pkt.src_addr(),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Packet<&'a T> {
    pub fn payload(&self) -> Result<&'a [u8]> {
        match &self {
            Packet::EthernetII(pkt) => Ok(pkt.payload()),
        }
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    pub fn set_protocol(&mut self, value: Protocol) {
        match self {
            Packet::EthernetII(pkt) => pkt.set_protocol(value),
        }
    }

    pub fn set_dest_addr(&mut self, addr: Address) {
        match self {
            Packet::EthernetII(pkt) => pkt.set_dest_addr(addr),
        }
    }

    pub fn set_src_addr(&mut self, addr: Address) {
        match self {
            Packet::EthernetII(pkt) => pkt.set_src_addr(addr),
        }
    }

    pub fn payload_mut(&mut self) -> &mut [u8] {
        match self {
            Packet::EthernetII(pkt) => pkt.payload_mut(),
        }
    }
}
