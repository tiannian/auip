use core::fmt::{self, Debug};
use core::format_args;

#[derive(Clone)]
pub struct Address(pub [u8; 6]);

impl Default for Address {
    fn default() -> Self {
        Self([0, 0, 0, 0, 0, 0])
    }
}

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

impl From<&[u8]> for Address {
    fn from(v: &[u8]) -> Self {
        let mut bytes = [0u8; 6];
        bytes.copy_from_slice(v);
        Self(bytes)
    }
}

impl Address {
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> Self {
        Self([a, b, c, d, e, f])
    }
}
