use super::{consts, HardwareAddress, Operation, ProtocolAddress};
use crate::{error::*, ip::ipv4::Address, mac};
use byteorder::{ByteOrder, NetworkEndian};
use core::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Packet<T> {
    buffer: T,
}

impl<T: AsRef<[u8]>> Display for Packet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Arp Packet:")?;
        f.write_fmt(format_args!(
            "{:?} to {:?}, Op: {:?}",
            self.source_hardware_address(),
            self.target_hardware_address(),
            self.operation()
        ))
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
        if len < field::OPER.end
            || len < field::target_protocol_address(self.hardware_len(), self.protocol_len()).end
        {
            Err(Error::WrongLengthForArpPacket)
        } else {
            Ok(())
        }
    }

    /// Return the source hardware address.
    pub fn source_hardware_address(&self) -> Result<HardwareAddress> {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::HTYPE]);

        if raw == consts::HARDWARE_ETHERNET {
            if self.hardware_len() == consts::HARDWARE_ETHERNET_LENGTH {
                let raw_addr = self.source_hardware_addr();
                let addr = mac::Address::from(raw_addr);
                Ok(HardwareAddress::Ethernet(addr))
            } else {
                Err(Error::WrongLengthForEthernetAddress)
            }
        } else {
            Ok(HardwareAddress::from(raw))
        }
    }

    /// Return the target hardware address.
    pub fn target_hardware_address(&self) -> Result<HardwareAddress> {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::HTYPE]);

        if raw == consts::HARDWARE_ETHERNET {
            if self.hardware_len() == consts::HARDWARE_ETHERNET_LENGTH {
                let raw_addr = self.target_hardware_addr();
                let addr = mac::Address::from(raw_addr);
                Ok(HardwareAddress::Ethernet(addr))
            } else {
                Err(Error::WrongLengthForEthernetAddress)
            }
        } else {
            Ok(HardwareAddress::from(raw))
        }
    }

    /// Return the protocol type field.
    pub fn source_protocol_address(&self) -> Result<ProtocolAddress> {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::PTYPE]);
        if raw == consts::HARDWARE_ETHERNET {
            if self.hardware_len() == consts::HARDWARE_ETHERNET_LENGTH {
                let raw_addr = self.source_protocol_addr();
                let addr = Address::from(raw_addr);
                Ok(ProtocolAddress::IPv4(addr))
            } else {
                Err(Error::WrongLengthForIpv4Address)
            }
        } else {
            Ok(ProtocolAddress::from(raw))
        }
    }

    /// Return the target protocol address.
    pub fn target_protocol_address(&self) -> Result<ProtocolAddress> {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::PTYPE]);
        if raw == consts::HARDWARE_ETHERNET {
            if self.hardware_len() == consts::HARDWARE_ETHERNET_LENGTH {
                let raw_addr = self.target_protocol_addr();
                let addr = Address::from(raw_addr);
                Ok(ProtocolAddress::IPv4(addr))
            } else {
                Err(Error::WrongLengthForIpv4Address)
            }
        } else {
            Ok(ProtocolAddress::from(raw))
        }
    }

    /// Return the operation field.
    pub fn operation(&self) -> Operation {
        let data = self.buffer.as_ref();
        let raw = NetworkEndian::read_u16(&data[field::OPER]);
        Operation::from(raw)
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    /// Set source hardware address
    pub fn set_source_map(
        &mut self,
        hd_addr: HardwareAddress,
        pc_addr: ProtocolAddress,
    ) -> Result<()> {
        self.set_hardware(&hd_addr);
        if let HardwareAddress::Ethernet(addr) = hd_addr {
            self.set_hardware_len(consts::HARDWARE_ETHERNET_LENGTH);
            self.set_source_hardware_addr(addr.as_ref());
        }

        self.set_protocol(&pc_addr);
        if let ProtocolAddress::IPv4(addr) = pc_addr {
            self.set_protocol_len(consts::PROTOCOL_IPV4_LENGTH);
            self.set_source_protocol_addr(addr.as_ref());
        }

        Ok(())
    }

    /// Set target hardware address
    pub fn set_target_map(
        &mut self,
        hd_addr: HardwareAddress,
        pc_addr: ProtocolAddress,
    ) -> Result<()> {
        self.set_hardware(&hd_addr);
        if let HardwareAddress::Ethernet(addr) = hd_addr {
            self.set_hardware_len(consts::HARDWARE_ETHERNET_LENGTH);
            self.set_target_hardware_addr(addr.as_ref());
        }

        self.set_protocol(&pc_addr);
        if let ProtocolAddress::IPv4(addr) = pc_addr {
            self.set_protocol_len(consts::PROTOCOL_IPV4_LENGTH);
            self.set_target_protocol_addr(addr.as_ref());
        }

        Ok(())
    }

    /// Set the operation field.
    pub fn set_operation(&mut self, value: Operation) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::OPER], value.into())
    }
}

impl<T: AsRef<[u8]>> Packet<T> {
    #[inline]
    fn hardware_len(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::HLEN]
    }

    #[inline]
    fn protocol_len(&self) -> u8 {
        let data = self.buffer.as_ref();
        data[field::PLEN]
    }

    #[inline]
    fn source_hardware_addr(&self) -> &[u8] {
        let data = self.buffer.as_ref();
        &data[field::source_hardware_address(self.hardware_len(), self.protocol_len())]
    }

    #[inline]
    fn source_protocol_addr(&self) -> &[u8] {
        let data = self.buffer.as_ref();
        &data[field::source_protocol_address(self.hardware_len(), self.protocol_len())]
    }

    #[inline]
    fn target_hardware_addr(&self) -> &[u8] {
        let data = self.buffer.as_ref();
        &data[field::target_hardware_address(self.hardware_len(), self.protocol_len())]
    }

    #[inline]
    fn target_protocol_addr(&self) -> &[u8] {
        let data = self.buffer.as_ref();
        &data[field::target_protocol_address(self.hardware_len(), self.protocol_len())]
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    /// Set the hardware type field.
    #[inline]
    fn set_hardware(&mut self, value: &HardwareAddress) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::HTYPE], value.into())
    }

    /// Set the protocol type field.
    #[inline]
    fn set_protocol(&mut self, value: &ProtocolAddress) {
        let data = self.buffer.as_mut();
        NetworkEndian::write_u16(&mut data[field::PTYPE], value.into())
    }

    /// Set the hardware length field.
    #[inline]
    fn set_hardware_len(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::HLEN] = value
    }

    /// Set the protocol length field.
    #[inline]
    fn set_protocol_len(&mut self, value: u8) {
        let data = self.buffer.as_mut();
        data[field::PLEN] = value
    }

    /// Set the source hardware address field.
    ///
    /// # Panics
    /// The function panics if `value` is not `self.hardware_len()` long.
    #[inline]
    fn set_source_hardware_addr(&mut self, value: &[u8]) {
        let (hardware_len, protocol_len) = (self.hardware_len(), self.protocol_len());
        let data = self.buffer.as_mut();
        data[field::source_hardware_address(hardware_len, protocol_len)].copy_from_slice(value)
    }

    /// Set the source protocol address field.
    ///
    /// # Panics
    /// The function panics if `value` is not `self.protocol_len()` long.
    #[inline]
    fn set_source_protocol_addr(&mut self, value: &[u8]) {
        let (hardware_len, protocol_len) = (self.hardware_len(), self.protocol_len());
        let data = self.buffer.as_mut();
        data[field::source_protocol_address(hardware_len, protocol_len)].copy_from_slice(value)
    }

    /// Set the target hardware address field.
    ///
    /// # Panics
    /// The function panics if `value` is not `self.hardware_len()` long.
    #[inline]
    fn set_target_hardware_addr(&mut self, value: &[u8]) {
        let (hardware_len, protocol_len) = (self.hardware_len(), self.protocol_len());
        let data = self.buffer.as_mut();
        data[field::target_hardware_address(hardware_len, protocol_len)].copy_from_slice(value)
    }

    /// Set the target protocol address field.
    ///
    /// # Panics
    /// The function panics if `value` is not `self.protocol_len()` long.
    #[inline]
    fn set_target_protocol_addr(&mut self, value: &[u8]) {
        let (hardware_len, protocol_len) = (self.hardware_len(), self.protocol_len());
        let data = self.buffer.as_mut();
        data[field::target_protocol_address(hardware_len, protocol_len)].copy_from_slice(value)
    }
}
