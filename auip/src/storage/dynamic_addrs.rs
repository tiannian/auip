use alloc::vec::Vec;
use auip_pkt::{
    layer2,
    layer3::{self, Cidr},
};

use crate::{AddrsStorage, Result, Error};

pub struct DynamicAddrsStorage {
    pub mac_addr: layer2::Address,
    pub ip_addrs: Vec<Cidr>,
}

impl AddrsStorage for DynamicAddrsStorage {
    fn mac_addr(&self) -> &layer2::Address {
        &self.mac_addr
    }

    fn set_mac_addr(&mut self, addr: layer2::Address) {
        self.mac_addr = addr;
    }

    fn add_ip_addr(&mut self, addr: layer3::Cidr) -> Result<()> {
        self.ip_addrs.push(addr);
        Ok(())
    }

    fn del_ip_addr(&mut self, addr: &layer3::Cidr) -> Result<()> {
        if let Ok(pos) = self.ip_addrs.binary_search(addr) {
            self.ip_addrs.remove(pos);

            Ok(())
        } else {
            Err(Error::IpAddrNotFound)
        }
        // self.ip_addrs.remo;
    }

    fn ip_addrs(&self) -> &[layer3::Cidr] {
        &self.ip_addrs
    }
}
