use alloc::collections::BTreeMap;
use auip_pkt::{layer2, layer3};

use crate::{ArpStorage, Result};

#[derive(Debug, Default)]
pub struct Arp {
    pub map: BTreeMap<layer3::ipv4::Address, layer2::Address>,
}

impl ArpStorage for Arp {
    fn set_map(&mut self, mac: layer2::Address, ip_addr: layer3::ipv4::Address) -> Result<()> {
        self.map.insert(ip_addr, mac);
        Ok(())
    }

    fn mac_addr(&self, ip_addr: &layer3::ipv4::Address) -> Result<Option<layer2::Address>> {
        Ok(self.map.get(ip_addr).copied())
    }
}
