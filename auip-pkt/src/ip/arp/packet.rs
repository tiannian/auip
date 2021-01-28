use super::Repr;
use super::{Hardware, Operation};
use crate::error::*;
use crate::mac::Protocol;
use byteorder::{ByteOrder, NetworkEndian};
use core::fmt::{self, Display};

#[derive(Debug, PartialEq, Clone)]
pub struct Packet<T: AsRef<[u8]>> {
    buffer: T,
}

impl<T: AsRef<[u8]>> Display for Packet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let repr = Repr::parse(self).unwrap();
        f.write_fmt(format_args!("{}", repr))
    }
}

mod field {
    use crate::utils::field::Field;

    pub const HTYPE: Field = 0..2;
    pub const PTYPE: Field = 2..4;
    pub const HLEN: usize = 4;
    pub const PLEN: usize = 5;
    pub const OPER: Field = 6..8;

    pub fn source_hardware_address(hardware_len: u8, _protocol_len: u8) -> Field {
        let start = OPER.end;
        start..(start + hardware_len as usize)
    }

    pub fn source_protocol_address(hardware_len: u8, protocol_len: u8) -> Field {
        let start = source_hardware_address(hardware_len, protocol_len).end;
        start..(start + protocol_len as usize)
    }

    pub fn target_hardware_address(hardware_len: u8, protocol_len: u8) -> Field {
        let start = source_protocol_address(hardware_len, protocol_len).end;
        start..(start + hardware_len as usize)
    }

    pub fn target_protocol_address(hardware_len: u8, protocol_len: u8) -> Field {
        let start = target_hardware_address(hardware_len, protocol_len).end;
        start..(start + protocol_len as usize)
    }
}

impl<T: AsRef<[u8]>> Packet<T> {
    pub fn new_unchecked(buffer: T) -> Packet<T> {
        Packet { buffer }
    }

    pub fn new_checked(buffer: T) -> Result<Packet<T>> {
        let packet = Self::new_unchecked(buffer);
        packet.check_len()?;
        Ok(packet)
    }

    fn check_len(&self) -> Result<()> {
        let len = self.buffer.as_ref().len();
        if len < field::OPER.end {
            Err(Error::Truncated)
        } else if len < field::target_protocol_address(self.hardware_len(), self.protocol_len()).end
        {
            Err(Error::Truncated)
        } else {
            Ok(())
        }
    }

    /// Return the hardware type field.
    pub fn hardware_type(&self) -> Hardware {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::HTYPE]);
        Hardware::from(raw)
    }

    /// Return the protocol type field.
    pub fn protocol_type(&self) -> Protocol {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::PTYPE]);
        Protocol::from(raw)
    }

    /// Return the hardware length field.
    pub fn hardware_len(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::HLEN]
    }

    /// Return the protocol length field.
    pub fn protocol_len(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::PLEN]
    }

    /// Return the operation field.
    pub fn operation(&self) -> Operation {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::OPER]);
        Operation::from(raw)
    }

    /// Return the source hardware address field.
    pub fn source_hardware_addr(&self) -> &[u8] {
        let data = self.buffer.as_ref();
        &data[field::source_hardware_address(self.hardware_len(), self.protocol_len())]
    }

    /// Return the source protocol address field.
    pub fn source_protocol_addr(&self) -> &[u8] {
        let data = self.buffer.as_ref();
        &data[field::source_protocol_address(self.hardware_len(), self.protocol_len())]
    }

    /// Return the target hardware address field.
    pub fn target_hardware_addr(&self) -> &[u8] {
        let data = self.buffer.as_ref();
        &data[field::target_hardware_address(self.hardware_len(), self.protocol_len())]
    }

    /// Return the target protocol address field.
    pub fn target_protocol_addr(&self) -> &[u8] {
        let data = self.buffer.as_ref();
        &data[field::target_protocol_address(self.hardware_len(), self.protocol_len())]
    }

    pub fn into_inner(self) -> T {
        self.buffer
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    /// Set the hardware type field.
    pub fn set_hardware_type(&mut self, value: Hardware) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::HTYPE], value.into())
    }

    /// Set the protocol type field.
    pub fn set_protocol_type(&mut self, value: Protocol) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::PTYPE], value.into())
    }

    /// Set the hardware length field.
    pub fn set_hardware_len(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::HLEN] = value
    }

    /// Set the protocol length field.
    pub fn set_protocol_len(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::PLEN] = value
    }

    /// Set the operation field.
    pub fn set_operation(&mut self, value: Operation) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::OPER], value.into())
    }

    /// Set the source hardware address field.
    ///
    /// # Panics
    /// The function panics if `value` is not `self.hardware_len()` long.
    pub fn set_source_hardware_addr(&mut self, value: &[u8]) {
        let (hardware_len, protocol_len) = (self.hardware_len(), self.protocol_len());
        let data = self.buffer.as_mut();
        data[field::source_hardware_address(hardware_len, protocol_len)].copy_from_slice(value)
    }

    /// Set the source protocol address field.
    ///
    /// # Panics
    /// The function panics if `value` is not `self.protocol_len()` long.
    pub fn set_source_protocol_addr(&mut self, value: &[u8]) {
        let (hardware_len, protocol_len) = (self.hardware_len(), self.protocol_len());
        let data = self.buffer.as_mut();
        data[field::source_protocol_address(hardware_len, protocol_len)].copy_from_slice(value)
    }

    /// Set the target hardware address field.
    ///
    /// # Panics
    /// The function panics if `value` is not `self.hardware_len()` long.
    pub fn set_target_hardware_addr(&mut self, value: &[u8]) {
        let (hardware_len, protocol_len) = (self.hardware_len(), self.protocol_len());
        let data = self.buffer.as_mut();
        data[field::target_hardware_address(hardware_len, protocol_len)].copy_from_slice(value)
    }

    /// Set the target protocol address field.
    ///
    /// # Panics
    /// The function panics if `value` is not `self.protocol_len()` long.
    pub fn set_target_protocol_addr(&mut self, value: &[u8]) {
        let (hardware_len, protocol_len) = (self.hardware_len(), self.protocol_len());
        let data = self.buffer.as_mut();
        data[field::target_protocol_address(hardware_len, protocol_len)].copy_from_slice(value)
    }
}
