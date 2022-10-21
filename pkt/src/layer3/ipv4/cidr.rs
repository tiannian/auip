use super::Address;
use crate::error::*;
use byteorder::{ByteOrder, NetworkEndian};

/// A specification of an IPv4 CIDR block, containing an address and a variable-length
/// subnet masking prefix length.
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default)]
pub struct Cidr {
    address: Address,
    prefix_len: u8,
}

impl Cidr {
    /// Create an IPv4 CIDR block from the given address and prefix length.
    ///
    /// # Panics
    /// This function panics if the prefix length is larger than 32.
    pub fn new(address: Address, prefix_len: u8) -> Cidr {
        assert!(prefix_len <= 32);
        Cidr {
            address,
            prefix_len,
        }
    }

    pub fn parse(s: &str) -> Result<Self> {
        let mut segments = s.split("/");

        let addr = segments.next().ok_or(Error::ParseIpv4CidrFailed)?;
        let mk = segments.next().ok_or(Error::ParseIpv4CidrFailed)?;

        let address = Address::parse(addr)?;
        let prefix_len = u8::from_str_radix(mk, 10)?;

        Ok(Self {
            address,
            prefix_len,
        })
    }

    /// Create an IPv4 CIDR block from the given address and network mask.
    pub fn from_netmask(addr: Address, netmask: Address) -> Result<Cidr> {
        let netmask = NetworkEndian::read_u32(&netmask.0[..]);
        if netmask.leading_zeros() == 0 && netmask.trailing_zeros() == netmask.count_zeros() {
            Ok(Cidr {
                address: addr,
                prefix_len: netmask.count_ones() as u8,
            })
        } else {
            Err(Error::IllegalNetmask)
        }
    }

    /// Return the address of this IPv4 CIDR block.
    pub fn address(&self) -> Address {
        self.address
    }

    /// Return the prefix length of this IPv4 CIDR block.
    pub fn prefix_len(&self) -> u8 {
        self.prefix_len
    }

    /// Return the network mask of this IPv4 CIDR.
    pub fn netmask(&self) -> Address {
        if self.prefix_len == 0 {
            return Address([0, 0, 0, 0]);
        }

        let number = 0xffffffffu32 << (32 - self.prefix_len);
        let data = [
            ((number >> 24) & 0xff) as u8,
            ((number >> 16) & 0xff) as u8,
            ((number >> 8) & 0xff) as u8,
            (number & 0xff) as u8,
        ];

        Address(data)
    }

    /// Return the broadcast address of this IPv4 CIDR.
    pub fn broadcast(&self) -> Option<Address> {
        let network = self.network();

        if network.prefix_len == 31 || network.prefix_len == 32 {
            return None;
        }

        let network_number = NetworkEndian::read_u32(&network.address.0[..]);
        let number = network_number | 0xffffffffu32 >> network.prefix_len;
        let data = [
            ((number >> 24) & 0xff) as u8,
            ((number >> 16) & 0xff) as u8,
            ((number >> 8) & 0xff) as u8,
            (number & 0xff) as u8,
        ];

        Some(Address(data))
    }

    /// Return the network block of this IPv4 CIDR.
    pub fn network(&self) -> Cidr {
        let mask = self.netmask().0;
        let network = [
            self.address.0[0] & mask[0],
            self.address.0[1] & mask[1],
            self.address.0[2] & mask[2],
            self.address.0[3] & mask[3],
        ];
        Cidr {
            address: Address(network),
            prefix_len: self.prefix_len,
        }
    }

    /// Query whether the subnetwork described by this IPv4 CIDR block contains
    /// the given address.
    pub fn contains_addr(&self, addr: &Address) -> bool {
        // right shift by 32 is not legal
        if self.prefix_len == 0 {
            return true;
        }

        let shift = 32 - self.prefix_len;
        let self_prefix = NetworkEndian::read_u32(self.address.as_bytes()) >> shift;
        let addr_prefix = NetworkEndian::read_u32(addr.as_bytes()) >> shift;
        self_prefix == addr_prefix
    }

    /// Query whether the subnetwork described by this IPv4 CIDR block contains
    /// the subnetwork described by the given IPv4 CIDR block.
    pub fn contains_subnet(&self, subnet: &Cidr) -> bool {
        self.prefix_len <= subnet.prefix_len && self.contains_addr(&subnet.address)
    }
}
