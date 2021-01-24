/// Ip protocol type.
#[derive(Debug, Clone)]
pub enum Protocol {
    HopByHop,
    Icmp,
    Igmp,
    Tcp,
    Udp,
    Ipv6Route,
    Ipv6Frag,
    Icmpv6,
    Ipv6NoNxt,
    Ipv6Opts,
    Unknown(u8),
}

impl From<u8> for Protocol {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Protocol::HopByHop,
            0x01 => Protocol::Icmp,
            0x02 => Protocol::Igmp,
            0x06 => Protocol::Tcp,
            0x11 => Protocol::Udp,
            0x2b => Protocol::Ipv6Route,
            0x2c => Protocol::Ipv6Frag,
            0x3a => Protocol::Icmpv6,
            0x3b => Protocol::Ipv6NoNxt,
            0x3c => Protocol::Ipv6Opts,
            _ => Protocol::Unknown(value),
        }
    }
}

impl From<Protocol> for u8 {
    fn from(value: Protocol) -> Self {
        match value {
            Protocol::HopByHop => 0x00,
            Protocol::Icmp => 0x01,
            Protocol::Igmp => 0x02,
            Protocol::Tcp => 0x06,
            Protocol::Udp => 0x11,
            Protocol::Ipv6Route => 0x2b,
            Protocol::Ipv6Frag => 0x2c,
            Protocol::Icmpv6 => 0x3a,
            Protocol::Ipv6NoNxt => 0x3b,
            Protocol::Ipv6Opts => 0x3c,
            Protocol::Unknown(v) => v,
        }
    }
}
