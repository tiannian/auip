use super::{EthernetRepr, Repr, Address};
use crate::ip;
use crate::ip::ipv4;
use crate::{FrameUp, ToRepr, Error, Result};
use byteorder::{ByteOrder, NetworkEndian};

#[derive(Debug, Clone)]
pub struct Packet<P: AsRef<[u8]>> {
    buffer: P,
}

impl<P: AsRef<[u8]>> AsRef<[u8]> for Packet<P> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

mod field {
    use crate::{Field, Rest};

    pub const DESTINATION: Field = 0..6;
    pub const SOURCE: Field = 6..12;
    pub const ETHERTYPE: Field = 12..14;
    pub const PAYLOAD: Rest = 14..;
}

impl<P: AsRef<[u8]>> Packet<P> {
    pub fn new_unchecked(buffer: P) -> Self {
        Self { buffer }
    }

    /// Shorthand for a combination of [new_unchecked] and [check_len].
    ///
    /// [new_unchecked]: #method.new_unchecked
    /// [check_len]: #method.check_len
    pub fn new_checked(buffer: P) -> Result<Self> {
        let packet = Self::new_unchecked(buffer);
        packet.check_len()?;
        Ok(packet)
    }

    /// Ensure that no accessor method will panic if called.
    /// Returns `Err(Error::Truncated)` if the buffer is too short.
    pub fn check_len(&self) -> Result<()> {
        let len = self.buffer.as_ref().len();
        if len < field::PAYLOAD.start {
            Err(Error::Truncated)
        } else {
            Ok(())
        }
    }

    /// Consumes the frame, returning the underlying buffer.
    pub fn into_inner(self) -> P {
        self.buffer
    }

    /// Return the length of a frame header.
    pub fn header_len() -> usize {
        field::PAYLOAD.start
    }

    /// Return the length of a buffer required to hold a packet with the payload
    /// of a given length.
    pub fn buffer_len(payload_len: usize) -> usize {
        field::PAYLOAD.start + payload_len
    }

    /// Return the destination address field.
    #[inline]
    pub fn dst_addr(&self) -> Address {
        let data = self.buffer.as_ref();
        (&data[field::DESTINATION]).into()
    }

    /// Return the source address field.
    #[inline]
    pub fn src_addr(&self) -> Address {
        let data = self.buffer.as_ref();
        (&data[field::SOURCE]).into()
    }

    // Return the EtherType field, without checking for 802.1Q.
    // #[inline]
    // pub fn ethertype(&self) -> EtherType {
    //     let data = self.buffer.as_ref();
    //     let raw = NetworkEndian::read_u16(&data[field::ETHERTYPE]);
    //     EtherType::from(raw)
    // }
}

impl<P: AsRef<[u8]>> ToRepr for Packet<P> {
    type Repr = Repr;

    fn to_repr(&self) -> Self::Repr {
        let inner = self.buffer.as_ref();
        let destination = &inner[field::DESTINATION];
        let source = &inner[field::SOURCE];
        let t = NetworkEndian::read_u16(&inner[field::ETHERTYPE]);
        Repr::Ethernet(EthernetRepr {
            destination: destination.into(),
            source: source.into(),
            t: t.into(),
        })
    }
}
