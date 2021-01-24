use core::fmt::{self, Debug};
use core::format_args;

#[derive(Clone)]
pub struct Address(pub [u8; 4]);

impl Default for Address {
    fn default() -> Self {
        Self([0, 0, 0, 0])
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.0;
        f.write_fmt(format_args!(
            "{}:{}:{}:{}",
            inner[0], inner[1], inner[2], inner[3]
        ))
    }
}

impl From<[u8; 4]> for Address {
    fn from(v: [u8; 4]) -> Self {
        Self(v)
    }
}

impl From<&[u8]> for Address {
    fn from(v: &[u8]) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(v);
        Self(bytes)
    }
}

impl Address {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self([a, b, c, d])
    }
}

// Add Cidr
