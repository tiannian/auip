use super::VlanId;

pub mod consts {
    pub const IPV4: u16 = 0x0800;
    pub const IPV6: u16 = 0x86DD;
    pub const ARP: u16 = 0x0806;
    pub const IEEE802_1Q: u16 = 0x8100;
    pub const Q_IN_Q: u16 = IEEE802_1Q;
    pub const IEEE802_3: u16 = 0x05DC;
}

/// Ethernet payload type for layer3
#[derive(Debug, Clone)]
pub enum Layer3Protocol {
    IPv4,

    IPv6,

    ARP,

    Unknown(u16),
}

impl From<u16> for Layer3Protocol {
    fn from(ty: u16) -> Self {
        match ty {
            consts::ARP => Self::ARP,
            consts::IPV4 => Self::IPv4,
            consts::IPV6 => Self::IPv6,
            _ => Self::Unknown(ty),
        }
    }
}

impl From<Layer3Protocol> for u16 {
    fn from(e: Layer3Protocol) -> u16 {
        match e {
            Layer3Protocol::ARP => consts::ARP,
            Layer3Protocol::IPv4 => consts::IPV4,
            Layer3Protocol::IPv6 => consts::IPV6,
            Layer3Protocol::Unknown(v) => v,
        }
    }
}

impl From<&Layer3Protocol> for u16 {
    fn from(e: &Layer3Protocol) -> u16 {
        match e {
            Layer3Protocol::ARP => consts::ARP,
            Layer3Protocol::IPv4 => consts::IPV4,
            Layer3Protocol::IPv6 => consts::IPV6,
            Layer3Protocol::Unknown(v) => *v,
        }
    }
}

/// Ethernet payload type.
#[derive(Debug, Clone)]
pub enum Protocol {
    Layer3Protocol(Layer3Protocol),

    IEEE8021Q(VlanId, Layer3Protocol),

    QinQ(VlanId, VlanId, Layer3Protocol),

    /// IEEE802.3
    Length(u16),

    Unknown(u16),
}

impl From<u16> for Protocol {
    fn from(ty: u16) -> Self {
        let layer3 = Layer3Protocol::from(ty);

        if let Layer3Protocol::Unknown(ty) = &layer3 {
            let ty = *ty;

            if ty < consts::IEEE802_3 {
                Self::Length(ty)
            } else {
                Self::Unknown(ty)
            }
        } else {
            Self::Layer3Protocol(layer3)
        }
    }
}

impl From<Protocol> for u16 {
    fn from(e: Protocol) -> u16 {
        match e {
            Protocol::Layer3Protocol(p) => p.into(),
            Protocol::IEEE8021Q(_, _) => consts::IEEE802_1Q,
            Protocol::QinQ(_, _, _) => consts::Q_IN_Q,
            Protocol::Length(v) => v,
            Protocol::Unknown(v) => v,
        }
    }
}

impl From<&Protocol> for u16 {
    fn from(e: &Protocol) -> u16 {
        match e {
            Protocol::Layer3Protocol(p) => p.into(),
            Protocol::IEEE8021Q(_, _) => consts::IEEE802_1Q,
            Protocol::QinQ(_, _, _) => consts::Q_IN_Q,
            Protocol::Length(v) => *v,
            Protocol::Unknown(v) => *v,
        }
    }
}
