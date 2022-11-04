use byteorder::{ByteOrder, NetworkEndian};

use crate::{utils::checksum, Error, IntoInner, Result};

use super::Message;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet<T> {
    buffer: T,
}

mod field {
    use crate::utils::field::Field;

    pub const TYPE: usize = 0;
    pub const CODE: usize = 1;
    pub const CHECKSUM: Field = 2..4;

    // pub const UNUSED: Field = 4..8;

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

    /// get Protocol
    pub fn protocol(&self) -> Message {
        let ty = self.msg_type();
        let code = self.msg_code();

        let mut msg = Message::from_type_code(ty, code);

        match &mut msg {
            Message::EchoRequest(e) => {
                e.ident = self.echo_ident();
                e.seq_no = self.echo_seq_no();
            }
            Message::EchoReply(e) => {
                e.ident = self.echo_ident();
                e.seq_no = self.echo_seq_no();
            }
            _ => {}
        }

        msg
    }

    /// Return the message type field.
    #[inline]
    fn msg_type(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::TYPE]
    }

    /// Return the message code field.
    #[inline]
    fn msg_code(&self) -> u8 {
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
    fn echo_ident(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::ECHO_IDENT])
    }

    /// Return the sequence number field (for echo request and reply packets).
    ///
    /// # Panics
    /// This function may panic if this packet is not an echo request or reply packet.
    #[inline]
    fn echo_seq_no(&self) -> u16 {
        let data = self.buffer.as_ref();
        NetworkEndian::read_u16(&data[field::ECHO_SEQNO])
    }

    /// Return the header length.
    /// The result depends on the value of the message type field.
    pub fn header_len(&self) -> usize {
        field::HEADER_END
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

    pub fn payload(&self) -> &[u8] {
        let data = self.buffer.as_ref();
        &data[self.header_len()..]
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    /// Set protocol
    pub fn set_protocol(&mut self, protocol: Message) {
        let (ty, code) = protocol.to_type_code();

        self.set_msg_type(ty);
        self.set_msg_code(code);

        match protocol {
            Message::EchoRequest(v) => {
                self.set_echo_ident(v.ident);
                self.set_echo_seq_no(v.seq_no);
            }
            Message::EchoReply(v) => {
                self.set_echo_ident(v.ident);
                self.set_echo_seq_no(v.seq_no);
            }
            _ => {}
        }
    }

    /// Set the message type field.
    #[inline]
    fn set_msg_type(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::TYPE] = value
    }

    /// Set the message code field.
    #[inline]
    fn set_msg_code(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::CODE] = value
    }

    /// Set the checksum field.
    #[inline]
    pub fn set_checksum(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::CHECKSUM], value)
    }

    /// Set the identifier field (for echo request and reply packets).
    ///
    /// # Panics
    /// This function may panic if this packet is not an echo request or reply packet.
    #[inline]
    fn set_echo_ident(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::ECHO_IDENT], value)
    }

    /// Set the sequence number field (for echo request and reply packets).
    ///
    /// # Panics
    /// This function may panic if this packet is not an echo request or reply packet.
    #[inline]
    fn set_echo_seq_no(&mut self, value: u16) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::ECHO_SEQNO], value)
    }

    /// Compute and fill in the header checksum.
    pub fn fill_checksum(&mut self) {
        self.set_checksum(0);
        let checksum = {
            let data = self.buffer.as_ref();
            !checksum::data(data)
        };
        self.set_checksum(checksum)
    }

    pub fn payload_mut(&mut self) -> &mut [u8] {
        let range = self.header_len()..;
        let data = self.buffer.as_mut();
        &mut data[range]
    }
}
