use super::ipv4;
// use super::address6;

#[derive(Debug, Clone)]
pub enum Address {
    Unspecified,
    IPv4(ipv4::Address),
    // IPv6(address6::Address),
}

impl Default for Address {
    fn default() -> Self {
        Self::Unspecified
    }
}

// TODO: add parse from str.
