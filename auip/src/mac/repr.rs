use super::Address;

#[derive(Debug, Clone)]
pub enum EthernetType {
    IPv4,
    IPv6,
    ARP,
    IEEE8021Q,
    Unknown(u16),
}

impl From<u16> for EthernetType {
    fn from(value: u16) -> Self {
        match value {
            0x0800 => EthernetType::IPv4,
            0x86DD => EthernetType::IPv6,
            0x0806 => EthernetType::ARP,
            0x8100 => EthernetType::IEEE8021Q,
            _ => EthernetType::Unknown(value),
        }
    }
}

impl From<EthernetType> for u16 {
    fn from(value: EthernetType) -> Self {
        match value {
            EthernetType::IPv4 => 0x0800,
            EthernetType::IPv6 => 0x86DD,
            EthernetType::ARP => 0x0806,
            EthernetType::IEEE8021Q => 0x8100,
            EthernetType::Unknown(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
pub struct EthernetRepr {
    pub destination: Address,
    pub source: Address,
    pub t: EthernetType,
}

pub enum Repr {
    Ethernet(EthernetRepr),
}
