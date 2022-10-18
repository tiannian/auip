use super::{Hardware, Operation, Packet};
use crate::error::*;
use crate::ip::ipv4;
use crate::mac;
use core::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Repr {
    /// An Ethernet and IPv4 Address Resolution Protocol packet.
    EthernetIpv4 {
        operation: Operation,
        source_hardware_addr: mac::Address,
        source_protocol_addr: ipv4::Address,
        target_hardware_addr: mac::Address,
        target_protocol_addr: ipv4::Address,
    },
}

impl Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Repr::EthernetIpv4 {
                operation, source_hardware_addr, source_protocol_addr, target_hardware_addr, target_protocol_addr
            } => {
                f.write_fmt(format_args!(
                    "Operation: {:?}\nSource Mac Address: {}\nSource Ip Address: {}\nTarget Mac Address: {}\nTarget Ip Address: {}",
                    operation, source_hardware_addr, source_protocol_addr, target_hardware_addr, target_protocol_addr
                ))
            }
        }
    }
}

impl Repr {
    /// Parse an Address Resolution Protocol packet and return a high-level representation,
    /// or return `Err(Error::Unrecognized)` if the packet is not recognized.
    pub fn parse<T: AsRef<[u8]>>(packet: &Packet<T>) -> Result<Repr> {
        match (
            packet.hardware_type(),
            packet.protocol_type(),
            packet.hardware_len(),
            packet.protocol_len(),
        ) {
            (Hardware::Ethernet, mac::Protocol::IPv4, 6, 4) => Ok(Repr::EthernetIpv4 {
                operation: packet.operation(),
                source_hardware_addr: mac::Address::from_bytes(packet.source_hardware_addr()),
                source_protocol_addr: ipv4::Address::from_bytes(packet.source_protocol_addr()),
                target_hardware_addr: mac::Address::from_bytes(packet.target_hardware_addr()),
                target_protocol_addr: ipv4::Address::from_bytes(packet.target_protocol_addr()),
            }),
            _ => Err(Error::Unrecognized),
        }
    }

    // /// Return the length of a packet that will be emitted from this high-level representation.
    // pub fn buffer_len(&self) -> usize {
    //     match *self {
    //         Repr::EthernetIpv4 { .. } => field::TPA(6, 4).end,
    //     }
    // }

    /// Emit a high-level representation into an Address Resolution Protocol packet.
    pub fn emit<T: AsRef<[u8]> + AsMut<[u8]>>(&self, packet: &mut Packet<T>) {
        match &*self {
            Repr::EthernetIpv4 {
                operation,
                source_hardware_addr,
                source_protocol_addr,
                target_hardware_addr,
                target_protocol_addr,
            } => {
                packet.set_hardware_type(Hardware::Ethernet);
                packet.set_protocol_type(mac::Protocol::IPv4);
                packet.set_hardware_len(6);
                packet.set_protocol_len(4);
                packet.set_operation(operation.clone());
                packet.set_source_hardware_addr(source_hardware_addr.as_bytes());
                packet.set_source_protocol_addr(source_protocol_addr.as_bytes());
                packet.set_target_hardware_addr(target_hardware_addr.as_bytes());
                packet.set_target_protocol_addr(target_protocol_addr.as_bytes());
            }
        }
    }
}
