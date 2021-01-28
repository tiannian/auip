//! EthernetII packet.

use super::{Address, Protocol};
use crate::error::*;
use byteorder::{ByteOrder, NetworkEndian};
use core::fmt::{self, Display};

/// Ethernet packet.
#[derive(Debug, Clone)]
pub struct Packet<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::utils::field::{Field, Rest};

    pub const DESTINATION: Field = 0..6;
    pub const SOURCE: Field = 6..12;
    pub const ETHERTYPE: Field = 12..14;
    pub const PAYLOAD: Rest = 14..;
}

impl<T: AsRef<[u8]>> Display for Packet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "\tDestination: {}\n\tSource: {}\n\tProtocol: {:?}\n\tPayload: {:?}",
            self.dest_addr(),
            self.src_addr(),
            self.protocol(),
            &self.as_ref()[field::PAYLOAD]
        ))
    }
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

    /// get buffer length for special payload length.
    pub fn buffer_len(payload_len: usize) -> usize {
        field::PAYLOAD.start + payload_len
    }

    /// get ethernet type.
    pub fn protocol(&self) -> Protocol {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::ETHERTYPE]);
        Protocol::from(raw)
    }

    pub fn into_inner(self) -> T {
        self.buffer
    }

    pub fn dest_addr(&self) -> Address {
        let inner = self.buffer.as_ref();
        (&inner[field::DESTINATION]).into()
    }

    pub fn src_addr(&self) -> Address {
        let inner = self.buffer.as_ref();
        (&inner[field::SOURCE]).into()
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Packet<&'a T> {
    pub fn payload(&self) -> &'a [u8] {
        let inner = self.buffer.as_ref();
        &inner[field::PAYLOAD]
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    /// set ethernet type.
    pub fn set_protocol(&mut self, value: Protocol) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::ETHERTYPE], value.into())
    }

    pub fn set_dest_addr(&mut self, addr: Address) {
        let data = self.buffer.as_mut();
        data[field::DESTINATION].copy_from_slice(addr.as_bytes());
    }

    pub fn set_src_addr(&mut self, addr: Address) {
        let data = self.buffer.as_mut();
        data[field::SOURCE].copy_from_slice(addr.as_bytes());
    }

    pub fn payload_mut(&mut self) -> &mut [u8] {
        let data = self.buffer.as_mut();
        &mut data[field::PAYLOAD]
    }
}
