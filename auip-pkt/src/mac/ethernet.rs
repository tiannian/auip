//! EthernetII packet.

use super::Address;
use crate::{
    DestAddr, DestAddrMut, Error, IntoInner, Payload, PayloadMut, Result, SrcAddr, SrcAddrMut,
};
use byteorder::{ByteOrder, NetworkEndian};

/// Ethernet payload type.
#[derive(Debug, Clone)]
pub enum EthernetType {
    IPv4,
    IPv6,
    ARP,
    IEEE8021Q,
    Unknown(u16),
}

impl From<u16> for EthernetType {
    fn from(value: u16) -> Self {
        match value {
            0x0800 => EthernetType::IPv4,
            0x86DD => EthernetType::IPv6,
            0x0806 => EthernetType::ARP,
            0x8100 => EthernetType::IEEE8021Q,
            _ => EthernetType::Unknown(value),
        }
    }
}

impl From<EthernetType> for u16 {
    fn from(value: EthernetType) -> Self {
        match value {
            EthernetType::IPv4 => 0x0800,
            EthernetType::IPv6 => 0x86DD,
            EthernetType::ARP => 0x0806,
            EthernetType::IEEE8021Q => 0x8100,
            EthernetType::Unknown(v) => v,
        }
    }
}

/// Ethernet packet.
#[derive(Debug, Clone)]
pub struct Packet<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::{Field, Rest};

    pub const DESTINATION: Field = 0..6;
    pub const SOURCE: Field = 6..12;
    pub const ETHERTYPE: Field = 12..14;
    pub const PAYLOAD: Rest = 14..;
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Packet<T> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

impl<T: AsRef<[u8]>> Packet<T> {
    /// new unchecked packet.
    pub fn new_unchecked(buffer: T) -> Packet<T> {
        Packet { buffer }
    }

    /// new checked packet.
    pub fn new_checked(buffer: T) -> Result<Packet<T>> {
        let packet = Self::new_unchecked(buffer);
        packet.check_len()?;
        Ok(packet)
    }

    fn check_len(&self) -> Result<()> {
        let len = self.buffer.as_ref().len();
        if len < field::PAYLOAD.start {
            Err(Error::Truncated)
        } else {
            Ok(())
        }
    }

    pub fn buffer_len(payload_len: usize) -> usize {
        field::PAYLOAD.start + payload_len
    }

    pub fn ethertype(&self) -> EthernetType {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::ETHERTYPE]);
        EthernetType::from(raw)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    pub fn set_ethertype(&mut self, value: EthernetType) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::ETHERTYPE], value.into())
    }
}

impl<T: AsRef<[u8]>> IntoInner for Packet<T> {
    type Inner = T;

    fn into_inner(self) -> Self::Inner {
        self.buffer
    }
}

impl<T: AsRef<[u8]>> DestAddr for Packet<T> {
    type Address = Address;

    fn dest_addr(&self) -> Self::Address {
        let inner = self.buffer.as_ref();
        Address::from_bytes(&inner[field::DESTINATION])
    }
}

impl<T: AsRef<[u8]>> SrcAddr for Packet<T> {
    type Address = Address;

    fn src_addr(&self) -> Self::Address {
        let inner = self.buffer.as_ref();
        Address::from_bytes(&inner[field::SOURCE])
    }
}

impl<T: AsRef<[u8]>> Payload for Packet<T> {
    type Payload = [u8];

    fn payload(&self) -> &Self::Payload {
        let inner = self.buffer.as_ref();
        &inner[field::PAYLOAD]
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> DestAddrMut for Packet<T> {
    type Address = Address;
    fn set_dest_addr(&mut self, addr: &Self::Address) {
        let data = self.buffer.as_mut();
        data[field::DESTINATION].copy_from_slice(addr.as_bytes())
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> SrcAddrMut for Packet<T> {
    type Address = Address;
    fn set_src_addr(&mut self, addr: &Self::Address) {
        let data = self.buffer.as_mut();
        data[field::SOURCE].copy_from_slice(addr.as_bytes())
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> PayloadMut for Packet<T> {
    type Payload = [u8];

    fn payload_mut(&mut self) -> &mut Self::Payload {
        let data = self.buffer.as_mut();
        &mut data[field::PAYLOAD]
    }
}
