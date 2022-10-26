use byteorder::{ByteOrder, NetworkEndian};

use crate::{
    layer3::{self, Address},
    utils::checksum,
    Error, IntoInner, Result,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet<T> {
    buffer: T,
}

mod field {
    use crate::utils::field::Field;

    pub const SRC_PORT: Field = 0..2;
    pub const DST_PORT: Field = 2..4;
    pub const LENGTH: Field = 4..6;
    pub const CHECKSUM: Field = 6..8;

    pub fn payload(length: u16) -> Field {
        CHECKSUM.end..(length as usize)
    }

    pub const HEADER_LEN: usize = CHECKSUM.end;
}

impl<T> IntoInner for Packet<T> {
    type Inner = T;

    fn into_inner(self) -> T {
        self.buffer
    }
}

impl<T: AsRef<[u8]>> Packet<T> {
    /// Imbue a raw octet buffer with UDP packet structure.
    pub fn new_unchecked(buffer: T) -> Packet<T> {
        Packet { buffer }
    }

    /// Shorthand for a combination of [new_unchecked] and [check_len].
    pub fn new_checked(buffer: T) -> Result<Packet<T>> {
        let packet = Self::new_unchecked(buffer);
        packet.check_len()?;
        Ok(packet)
    }

    /// Ensure that no accessor method will panic if called.
    fn check_len(&self) -> Result<()> {
        let buffer_len = self.buffer.as_ref().len();
        if buffer_len < field::HEADER_LEN {
            Err(Error::WrongLengthForBufferLength)
        } else {
            let field_len = self.len() as usize;
            if buffer_len < field_len || field_len < field::HEADER_LEN {
                Err(Error::WrongLengthForBufferLength)
            } else {
                Ok(())
            }
        }
    }

    /// Return the source port field.
    #[inline]
    pub fn src_port(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::SRC_PORT])
    }

    /// Return the destination port field.
    #[inline]
    pub fn dst_port(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::DST_PORT])
    }

    /// Return the length field.
    #[inline]
    pub fn len(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::LENGTH])
    }

    /// Return the checksum field.
    #[inline]
    pub fn checksum(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::CHECKSUM])
    }

    /// Validate the packet checksum.
    pub fn verify_checksum(
        &self,
        src_addr: &layer3::Address,
        dst_addr: &layer3::Address,
    ) -> Result<bool> {
        // From the RFC:
        // > An all zero transmitted checksum value means that the transmitter
        // > generated no checksum (for debugging or for higher level protocols
        // > that don't care).
        if self.checksum() == 0 {
            return Ok(true);
        }

        let data = self.buffer.as_ref();
        Ok(checksum::combine(&[
            checksum::pseudo_ip_header(
                src_addr,
                dst_addr,
                layer3::Protocol::Udp.into(),
                self.len() as u32,
            )?,
            checksum::data(&data[..self.len() as usize]),
        ]) == !0)
    }

    pub fn payload(&self) -> &[u8] {
        let length = self.len();
        let data = self.buffer.as_ref();
        &data[field::payload(length)]
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    /// Set the source port field.
    #[inline]
    pub fn set_src_port(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::SRC_PORT], value)
    }

    /// Set the destination port field.
    #[inline]
    pub fn set_dst_port(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::DST_PORT], value)
    }

    /// Set the length field.
    #[inline]
    pub fn set_len(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::LENGTH], value)
    }

    /// Set the checksum field.
    #[inline]
    pub fn set_checksum(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::CHECKSUM], value)
    }

    /// Compute and fill in the header checksum.
    pub fn fill_checksum(&mut self, src_addr: &Address, dst_addr: &Address) -> Result<()> {
        self.set_checksum(0);
        let checksum = {
            let data = self.buffer.as_ref();
            !checksum::combine(&[
                checksum::pseudo_ip_header(
                    src_addr,
                    dst_addr,
                    layer3::Protocol::Udp.into(),
                    self.len() as u32,
                )?,
                checksum::data(&data[..self.len() as usize]),
            ])
        };
        // UDP checksum value of 0 means no checksum; if the checksum really is zero,
        // use all-ones, which indicates that the remote end must verify the checksum.
        // Arithmetically, RFC 1071 checksums of all-zeroes and all-ones behave identically,
        // so no action is necessary on the remote end.
        self.set_checksum(if checksum == 0 { 0xffff } else { checksum });
        Ok(())
    }

    /// Return a mutable pointer to the payload.
    #[inline]
    pub fn payload_mut(&mut self) -> &mut [u8] {
        let length = self.len();
        let data = self.buffer.as_mut();
        &mut data[field::payload(length)]
    }
}
