/// Ethernet payload type.
#[derive(Debug, Clone)]
pub enum Protocol {
    IPv4,
    IPv6,
    ARP,
    IEEE8021Q,
    Unknown(u16),
}

impl From<u16> for Protocol {
    fn from(value: u16) -> Self {
        match value {
            0x0800 => Protocol::IPv4,
            0x86DD => Protocol::IPv6,
            0x0806 => Protocol::ARP,
            0x8100 => Protocol::IEEE8021Q,
            _ => Protocol::Unknown(value),
        }
    }
}

impl From<Protocol> for u16 {
    fn from(value: Protocol) -> Self {
        match value {
            Protocol::IPv4 => 0x0800,
            Protocol::IPv6 => 0x86DD,
            Protocol::ARP => 0x0806,
            Protocol::IEEE8021Q => 0x8100,
            Protocol::Unknown(v) => v,
        }
    }
}
