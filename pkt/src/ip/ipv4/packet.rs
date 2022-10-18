use super::Address;
use crate::error::*;
use crate::ip;
use crate::utils::checksum;
use byteorder::{ByteOrder, NetworkEndian};

#[derive(Debug, PartialEq, Clone)]
pub struct Packet<T: AsRef<[u8]>> {
    buffer: T,
}

mod field {
    use crate::utils::field::Field;

    pub const VER_IHL: usize = 0;
    pub const DSCP_ECN: usize = 1;
    pub const LENGTH: Field = 2..4;
    pub const IDENT: Field = 4..6;
    pub const FLG_OFF: Field = 6..8;
    pub const TTL: usize = 8;
    pub const PROTOCOL: usize = 9;
    pub const CHECKSUM: Field = 10..12;
    pub const SRC_ADDR: Field = 12..16;
    pub const DST_ADDR: Field = 16..20;
    pub const OPTIONS: u8 = 20;
}

impl<T: AsRef<[u8]>> IntoInner for Packet<T> {
    type Inner = T;

    fn into_inner(self) -> Self::Inner {
        self.buffer
    }
}

impl<T: AsRef<[u8]>> DestAddr for Packet<T> {
    type Address = Address;

    fn dest_addr(&self) -> Result<Self::Address> {
        let data = self.buffer.as_ref();
        Ok((&data[field::DST_ADDR]).into())
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> DestAddrMut for Packet<T> {
    type Address = Address;

    fn set_dest_addr(&mut self, addr: &Self::Address) -> Result<()> {
        let data = self.buffer.as_mut();
        data[field::DST_ADDR].copy_from_slice(addr.as_bytes());
        Ok(())
    }
}

impl<T: AsRef<[u8]>> SrcAddr for Packet<T> {
    type Address = Address;

    fn src_addr(&self) -> Result<Self::Address> {
        let data = self.buffer.as_ref();
        Ok((&data[field::SRC_ADDR]).into())
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> SrcAddrMut for Packet<T> {
    type Address = Address;

    fn set_src_addr(&mut self, addr: &Self::Address) -> Result<()> {
        let data = self.buffer.as_mut();
        Ok(data[field::SRC_ADDR].copy_from_slice(addr.as_bytes()))
    }
}

// impl<T: AsRef<[u8]>> Payload for Packet<T> {
//     type Payload = [u8];

//     fn payload(&self) -> Result<&Self::Payload> {
//         let inner = self.buffer.as_ref();
//         let payload_field = (self.header_len() as usize)..;
//         Ok(&inner[payload_field])
//     }
// }

impl<T: AsRef<[u8]> + AsMut<[u8]>> PayloadMut for Packet<T> {
    type Payload = [u8];

    fn payload_mut(&mut self) -> Result<&mut Self::Payload> {
        let payload_field = (self.header_len() as usize)..;
        let data = self.buffer.as_mut();
        Ok(&mut data[payload_field])
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
        if len < field::DST_ADDR.end {
            Err(Error::Truncated)
        } else if len < self.header_len() as usize {
            Err(Error::Truncated)
        } else if self.header_len() as u16 > self.total_len() {
            Err(Error::Malformed)
        } else if len < self.total_len() as usize {
            Err(Error::Truncated)
        } else {
            Ok(())
        }
    }

    /// Return the header length, in octets.
    pub fn header_len(&self) -> u8 {
        let data = self.buffer.as_ref();
        (data[field::VER_IHL] & 0x0f) * 4
    }

    /// Return the total length field.
    pub fn total_len(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::LENGTH])
    }

    /// Return the total length field.
    pub fn option_len(&self) -> u8 {
        let header_len = self.header_len();
        header_len - field::OPTIONS
    }

    /// Return the version field.
    pub fn version(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::VER_IHL] >> 4
    }

    /// Return the Differential Services Code Point field.
    pub fn dscp(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::DSCP_ECN] >> 2
    }

    /// Return the Explicit Congestion Notification field.
    pub fn ecn(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::DSCP_ECN] & 0x03
    }

    /// Return the fragment identification field.
    pub fn ident(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::IDENT])
    }

    /// Return the "don't fragment" flag.
    pub fn dont_frag(&self) -> bool {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::FLG_OFF]) & 0x4000 != 0
    }
    /// Return the "more fragments" flag.
    pub fn more_frags(&self) -> bool {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::FLG_OFF]) & 0x2000 != 0
    }

    /// Return the fragment offset, in octets.
    pub fn frag_offset(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::FLG_OFF]) << 3
    }

    /// Return the time to live field.
    pub fn hop_limit(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::TTL]
    }

    /// Return the header checksum field.
    pub fn checksum(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::CHECKSUM])
    }

    /// Return the protocol field.
    pub fn protocol(&self) -> ip::Protocol {
        let data = self.buffer.as_ref();
        ip::Protocol::from(data[field::PROTOCOL])
    }

    /// Validate the header checksum.
    ///
    /// # Fuzzing
    /// This function always returns `true` when fuzzing.
    pub fn verify_checksum(&self) -> bool {
        if cfg!(fuzzing) {
            return true;
        }

        let data = self.buffer.as_ref();
        checksum::data(&data[..self.header_len() as usize]) == !0
    }
}
