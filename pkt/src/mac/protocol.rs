use super::VlanId;

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

    IEEE8021Q(VlanId),

    QinQ(VlanId, VlanId),

    /// IEEE802.3
    Length(u16),

    Unknown(u16),
}

impl From<u16> for Protocol {
    fn from(ty: u16) -> Self {
        if ty < consts::IEEE802_3 {
            Self::Length(ty)
        } else {
            match ty {
                consts::ARP => Self::ARP,
                consts::IPV4 => Self::IPv4,
                consts::IPV6 => Self::IPv6,
                _ => Self::Unknown(ty),
            }
        }
    }
}

impl From<Protocol> for u16 {
    fn from(e: Protocol) -> u16 {
        match e {
            Protocol::ARP => consts::ARP,
            Protocol::IPv4 => consts::IPV4,
            Protocol::IPv6 => consts::IPV6,
            Protocol::IEEE8021Q(_) => consts::IEEE802_1Q,
            Protocol::QinQ(_, _) => consts::Q_IN_Q,
            Protocol::Length(v) => v,
            Protocol::Unknown(v) => v,
        }
    }
}

impl From<&Protocol> for u16 {
    fn from(e: &Protocol) -> u16 {
        match e {
            Protocol::ARP => consts::ARP,
            Protocol::IPv4 => consts::IPV4,
            Protocol::IPv6 => consts::IPV6,
            Protocol::IEEE8021Q(_) => consts::IEEE802_1Q,
            Protocol::QinQ(_, _) => consts::Q_IN_Q,
            Protocol::Length(v) => *v,
            Protocol::Unknown(v) => *v,
        }
    }
}
