use core::fmt::{self, Debug};
use core::format_args;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Address(pub [u8; 6]);

impl Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.0;
        f.write_fmt(format_args!(
            "{:02x}-{:02x}-{:02x}-{:02x}-{:02x}-{:02x}",
            inner[0], inner[1], inner[2], inner[3], inner[4], inner[5]
        ))
    }
}

impl From<[u8; 6]> for Address {
    fn from(v: [u8; 6]) -> Self {
        Self(v)
    }
}

impl From<Address> for [u8; 6] {
    fn from(v: Address) -> Self {
        v.0
    }
}

impl From<&[u8]> for Address {
    fn from(v: &[u8]) -> Self {
        Address::from_bytes(v)
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.as_bytes()
    }
}

impl Address {
    pub const BROADCAST: Address = Address([0xff; 6]);

    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self([a, b, c, d, e, f])
    }

    pub fn from_bytes(v: &[u8]) -> Self {
        let mut bytes = [0u8; 6];
        bytes.copy_from_slice(v);
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    pub fn is_unicast(&self) -> bool {
        !(self.is_broadcast() || self.is_multicast())
    }

    pub fn is_broadcast(&self) -> bool {
        *self == Self::BROADCAST
    }

    pub fn is_multicast(&self) -> bool {
        self.0[0] & 0x01 != 0
    }

    pub fn is_local(&self) -> bool {
        self.0[0] & 0x02 != 0
    }
}
