use auip_pkt::{layer2, layer3};

use crate::{AddrsStorage, Error, Result};

pub struct Addrs<const IP_ADDR_NUM: usize> {
    pub mac_addr: layer2::Address,
    pub ip_addrs: [layer3::Cidr; IP_ADDR_NUM],
}

impl<const IP_ADDR_NUM: usize> Default for Addrs<IP_ADDR_NUM> {
    fn default() -> Self {
        let ip_addrs = [Default::default(); IP_ADDR_NUM];

        Self {
            mac_addr: Default::default(),
            ip_addrs,
        }
    }
}

impl<const IP_ADDR_NUM: usize> AddrsStorage for Addrs<IP_ADDR_NUM> {
    fn mac_addr(&self) -> &layer2::Address {
        &self.mac_addr
    }

    fn has_ip_addr(&self, ip_addr: &layer3::Address) -> bool {
        let addrs = &self.ip_addrs;

        addrs
            .binary_search_by_key(ip_addr, |a| *a.address())
            .is_ok()
    }
}

impl<const IP_ADDR_NUM: usize> Addrs<IP_ADDR_NUM> {
    pub fn set_mac_addr(&mut self, addr: layer2::Address) {
        self.mac_addr = addr;
    }

    pub fn add_ip_addr(&mut self, addr: layer3::Cidr) -> Result<()> {
        let empty = layer3::Cidr::default();

        if let Ok(pos) = self.ip_addrs.binary_search(&empty) {
            self.ip_addrs[pos] = addr;
            Ok(())
        } else {
            Err(Error::NoSpaceForAddrsStorage)
        }
    }

    pub fn del_ip_addr(&mut self, addr: &layer3::Cidr) -> Result<()> {
        if let Ok(pos) = self.ip_addrs.binary_search(addr) {
            self.ip_addrs[pos] = layer3::Cidr::default();
            Ok(())
        } else {
            Err(Error::IpAddrNotFound)
        }
    }

    pub fn ip_addrs(&self) -> &[layer3::Cidr] {
        &self.ip_addrs
    }
}
