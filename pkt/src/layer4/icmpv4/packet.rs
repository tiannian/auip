use byteorder::{ByteOrder, NetworkEndian};

use crate::{utils::checksum, Error, IntoInner, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet<T> {
    buffer: T,
}

mod field {
    use crate::utils::field::Field;

    pub const TYPE: usize = 0;
    pub const CODE: usize = 1;
    pub const CHECKSUM: Field = 2..4;

    pub const UNUSED: Field = 4..8;

    pub const ECHO_IDENT: Field = 4..6;
    pub const ECHO_SEQNO: Field = 6..8;

    pub const HEADER_END: usize = 8;
}

impl<T> IntoInner for Packet<T> {
    type Inner = T;

    fn into_inner(self) -> Self::Inner {
        self.buffer
    }
}

impl<T: AsRef<[u8]>> Packet<T> {
    /// Imbue a raw octet buffer with ICMPv4 packet structure.
    pub fn new_unchecked(buffer: T) -> Packet<T> {
        Packet { buffer }
    }

    /// Shorthand for a combination of [new_unchecked] and [check_len].
    ///
    /// [new_unchecked]: #method.new_unchecked
    /// [check_len]: #method.check_len
    pub fn new_checked(buffer: T) -> Result<Packet<T>> {
        let packet = Self::new_unchecked(buffer);
        packet.check_len()?;
        Ok(packet)
    }

    /// Ensure that no accessor method will panic if called.
    /// Returns `Err(Error)` if the buffer is too short.
    ///
    /// The result of this check is invalidated by calling [set_header_len].
    ///
    /// [set_header_len]: #method.set_header_len
    pub fn check_len(&self) -> Result<()> {
        let len = self.buffer.as_ref().len();
        if len < field::HEADER_END {
            Err(Error::WrongLengthForBufferLength)
        } else {
            Ok(())
        }
    }

    /// Return the message type field.
    #[inline]
    pub fn msg_type(&self) -> Message {
        let data = self.buffer.as_ref();
        Message::from(data[field::TYPE])
    }

    /// Return the message code field.
    #[inline]
    pub fn msg_code(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::CODE]
    }

    /// Return the checksum field.
    #[inline]
    pub fn checksum(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::CHECKSUM])
    }

    /// Return the identifier field (for echo request and reply packets).
    ///
    /// # Panics
    /// This function may panic if this packet is not an echo request or reply packet.
    #[inline]
    pub fn echo_ident(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::ECHO_IDENT])
    }

    /// Return the sequence number field (for echo request and reply packets).
    ///
    /// # Panics
    /// This function may panic if this packet is not an echo request or reply packet.
    #[inline]
    pub fn echo_seq_no(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::ECHO_SEQNO])
    }

    /// Return the header length.
    /// The result depends on the value of the message type field.
    pub fn header_len(&self) -> usize {
        match self.msg_type() {
            Message::EchoRequest => field::ECHO_SEQNO.end,
            Message::EchoReply => field::ECHO_SEQNO.end,
            Message::DstUnreachable => field::UNUSED.end,
            _ => field::UNUSED.end, // make a conservative assumption
        }
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
        checksum::data(data) == !0
    }
}
