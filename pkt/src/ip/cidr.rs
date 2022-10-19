use super::ipv4;
use super::Address;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Cidr {
    address: Address,
    prefix_len: u8,
}

impl Cidr {
    pub fn new(address: Address, prefix_len: u8) -> Cidr {
        Cidr {
            address,
            prefix_len,
        }
    }

    pub fn prefix_len(&self) -> u8 {
        self.prefix_len
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn contains_addr(&self, addr: &Address) -> bool {
        match (self.address, addr) {
            (Address::Ipv4(v1), Address::Ipv4(v2)) => {
                let cidr = ipv4::Cidr::new(v1, self.prefix_len);
                cidr.contains_addr(v2)
            }
            _ => false,
        }
    }

    pub fn contains_subnet(&self, subnet: &Cidr) -> bool {
        match (self.address, subnet.address()) {
            (Address::Ipv4(v1), Address::Ipv4(v2)) => {
                let cidr1 = ipv4::Cidr::new(v1, self.prefix_len);
                let cidr2 = ipv4::Cidr::new(*v2, subnet.prefix_len());
                cidr1.contains_subnet(&cidr2)
            }
            _ => false,
        }
    }
}
