use super::{Hardware, Operation};
use crate::error::*;
use crate::mac::Protocol;
use crate::prelude::*;
use crate::{ip::ipv4, mac};
use byteorder::{ByteOrder, NetworkEndian};

#[derive(Debug, PartialEq, Clone)]
pub struct Packet<T: AsRef<[u8]>> {
    buffer: T,
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
        start..(start + protocol_len as usize)
    }

    pub fn target_protocol_address(hardware_len: u8, protocol_len: u8) -> Field {
        let start = target_hardware_address(hardware_len, protocol_len).end;
        start..(start + protocol_len as usize)
    }
}

impl<T: AsRef<[u8]>> Packet<T> {
    /// Imbue a raw octet buffer with ARP packet structure.
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
    /// Returns `Err(Error::Truncated)` if the buffer is too short.
    ///
    /// The result of this check is invalidated by calling [set_hardware_len] or
    /// [set_protocol_len].
    ///
    /// [set_hardware_len]: #method.set_hardware_len
    /// [set_protocol_len]: #method.set_protocol_len
    #[allow(clippy::if_same_then_else)]
    pub fn check_len(&self) -> Result<()> {
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
    #[inline]
    pub fn hardware_type(&self) -> Hardware {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::HTYPE]);
        Hardware::from(raw)
    }

    /// Return the protocol type field.
    #[inline]
    pub fn protocol_type(&self) -> Protocol {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::PTYPE]);
        Protocol::from(raw)
    }

    /// Return the hardware length field.
    #[inline]
    pub fn hardware_len(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::HLEN]
    }

    /// Return the protocol length field.
    #[inline]
    pub fn protocol_len(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::PLEN]
    }

    /// Return the operation field.
    #[inline]
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
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    /// Set the hardware type field.
    #[inline]
    pub fn set_hardware_type(&mut self, value: Hardware) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::HTYPE], value.into())
    }

    /// Set the protocol type field.
    #[inline]
    pub fn set_protocol_type(&mut self, value: Protocol) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::PTYPE], value.into())
    }

    /// Set the hardware length field.
    #[inline]
    pub fn set_hardware_len(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::HLEN] = value
    }

    /// Set the protocol length field.
    #[inline]
    pub fn set_protocol_len(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::PLEN] = value
    }

    /// Set the operation field.
    #[inline]
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

impl<T: AsRef<[u8]>> IntoInner for Packet<T> {
    type Inner = T;

    fn into_inner(self) -> Self::Inner {
        self.buffer
    }
}

impl<T: AsRef<[u8]>> DestAddr for Packet<T> {
    type Address = (ipv4::Address, mac::Address);

    fn dest_addr(&self) -> Result<Self::Address> {
        if self.hardware_len() == 6 && self.protocol_len() == 4 {
            let ip_addr = self.target_hardware_addr().into();
            let mac_addr = self.target_protocol_addr().into();
            Ok((ip_addr, mac_addr))
        } else {
            Err(Error::Illegal)
        }
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> DestAddrMut for Packet<T> {
    type Address = (ipv4::Address, mac::Address);
    fn set_dest_addr(&mut self, addr: &Self::Address) -> Result<()> {
        // add some code here
        let (ipv4_addr, mac_addr) = addr;
        self.set_hardware_type(Hardware::Ethernet);
        self.set_target_hardware_addr(mac_addr.as_ref());
        self.set_protocol_type(Protocol::IPv4);
        self.set_target_protocol_addr(ipv4_addr.as_ref());
        Ok(())
    }
}

impl<T: AsRef<[u8]>> SrcAddr for Packet<T> {
    type Address = (ipv4::Address, mac::Address);

    fn src_addr(&self) -> Result<Self::Address> {
        if self.hardware_len() == 6 && self.protocol_len() == 4 {
            let ip_addr = self.source_hardware_addr().into();
            let mac_addr = self.source_protocol_addr().into();
            Ok((ip_addr, mac_addr))
        } else {
            Err(Error::Illegal)
        }
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> SrcAddrMut for Packet<T> {
    type Address = (ipv4::Address, mac::Address);
    fn set_src_addr(&mut self, addr: &Self::Address) -> Result<()> {
        // add some code here
        let (ipv4_addr, mac_addr) = addr;
        self.set_hardware_type(Hardware::Ethernet);
        self.set_source_hardware_addr(mac_addr.as_ref());
        self.set_protocol_type(Protocol::IPv4);
        self.set_source_protocol_addr(ipv4_addr.as_ref());
        Ok(())
    }
}
