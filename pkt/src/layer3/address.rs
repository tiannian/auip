use super::ipv4;

// #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
// pub enum Version {
//     Unspecified,
//     Ipv4,
//     Ipv6,
// }

/// An internetworking address.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Address {
    /// An unspecified address.
    /// May be used as a placeholder for storage where the address is not assigned yet.
    Unspecified,
    /// An IPv4 address.
    Ipv4(ipv4::Address),
    // An IPv6 address.
    // Ipv6(Ipv6Address),
}

impl Default for Address {
    fn default() -> Self {
        Self::Unspecified
    }
}
