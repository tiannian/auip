use crate::{mac, ip::ipv4::Address};

pub mod consts {
    pub const HARDWARE_ETHERNET: u16 = 1;

    pub const HARDWARE_ETHERNET_LENGTH: u8 = 6;

    pub const OPERATION_REQUEST: u16 = 1;
    pub const OPERATION_REPLAY: u16 = 2;

    pub const PROTOCOL_IPV4: u16 = 0x0800;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HardwareAddress {
    Ethernet(mac::Address),
    Unknown(u16),
}

impl From<u16> for HardwareAddress {
    fn from(v: u16) -> Self {
        match v {
            _ => Self::Unknown(v),
        }
    }
}

impl From<HardwareAddress> for u16 {
    fn from(v: HardwareAddress) -> u16 {
        match v {
            HardwareAddress::Ethernet(_) => consts::HARDWARE_ETHERNET,
            HardwareAddress::Unknown(a) => a,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Request,
    Reply,
    Unknown(u16),
}

impl From<u16> for Operation {
    fn from(v: u16) -> Self {
        match v {
            consts::OPERATION_REQUEST => Operation::Request,
            consts::OPERATION_REPLAY => Operation::Reply,
            _ => Operation::Unknown(v),
        }
    }
}

impl From<Operation> for u16 {
    fn from(v: Operation) -> u16 {
        match v {
            Operation::Request => consts::OPERATION_REQUEST,
            Operation::Reply => consts::OPERATION_REPLAY,
            Operation::Unknown(a) => a,
        }
    }
}

/// Ethernet payload type.
#[derive(Debug, Clone)]
pub enum ProtocolAddress {
    IPv4(Address),

    Unknown(u16),
}

impl From<u16> for ProtocolAddress {
    fn from(ty: u16) -> Self {
        match ty {
            _ => Self::Unknown(ty),
        }
    }
}

impl From<ProtocolAddress> for u16 {
    fn from(e: ProtocolAddress) -> u16 {
        match e {
            ProtocolAddress::IPv4(_) => consts::PROTOCOL_IPV4,
            ProtocolAddress::Unknown(v) => v,
        }
    }
}

impl From<&ProtocolAddress> for u16 {
    fn from(e: &ProtocolAddress) -> u16 {
        match e {
            ProtocolAddress::IPv4(_) => consts::PROTOCOL_IPV4,
            ProtocolAddress::Unknown(v) => *v,
        }
    }
}
