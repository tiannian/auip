pub mod consts {
    pub const IPV4: u16 = 0x0800;
    pub const IPV6: u16 = 0x86DD;
    pub const ARP: u16 = 0x0806;
    pub const IEEE802_1Q: u16 = 0x8100;
    pub const Q_IN_Q: u16 = IEEE802_1Q;
    pub const IEEE802_3: u16 = 0x05DC;
}


/// Ethernet payload type.
#[derive(Debug, Clone)]
pub enum Protocol {

    IPv4,

    IPv6,

    ARP,

    IEEE8021Q,

    QinQ,

    /// IEEE802.3
    Length(u16),

    Unknown(u16),
}

impl From<u16> for Protocol {
    fn from(value: u16) -> Self {
        if value < consts::IEEE802_3 {
            Protocol::Length(value)
        } else {
            match value {
                consts::IPV4 => Protocol::IPv4,
                consts::IPV6 => Protocol::IPv6,
                consts::ARP => Protocol::ARP,
                consts::IEEE802_1Q => Protocol::IEEE8021Q,
                _ => Protocol::Unknown(value),
            }
        }
    }
}

impl From<Protocol> for u16 {
    fn from(value: Protocol) -> Self {
        match value {
            Protocol::IPv4 => consts::IPV4,
            Protocol::IPv6 => consts::IPV6,
            Protocol::ARP => consts::ARP,
            Protocol::IEEE8021Q => consts::Q_IN_Q,
            Protocol::QinQ => consts::Q_IN_Q,
            Protocol::Unknown(v) => v,
            Protocol::Length(v) => v,
        }
    }
}
