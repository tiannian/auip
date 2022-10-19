use core::fmt::{self, Debug, Display};
use core::format_args;
use core::ops::Deref;

use byteorder::{ByteOrder, NetworkEndian};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Address(pub [u8; 6]);

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inner = self.0;
        f.write_fmt(format_args!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
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
        self.as_bytes()
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

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct VlanId(pub u16);

impl From<u16> for VlanId {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl From<VlanId> for u16 {
    fn from(v: VlanId) -> Self {
        v.0
    }
}

impl AsRef<u16> for VlanId {
    fn as_ref(&self) -> &u16 {
        &self.0
    }
}

impl Deref for VlanId {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl VlanId {
    pub fn from_bytes_unchecked(buf: &[u8]) -> Self {
        let mut bytes = [0u8; 2];

        bytes[0] = 0x0F & buf[0];
        bytes[1] = buf[1];

        VlanId(NetworkEndian::read_u16(&bytes))
    }
}
