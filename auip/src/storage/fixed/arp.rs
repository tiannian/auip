use auip_pkt::{layer2, layer3};

use crate::{ArpStorage, Error, Result};

pub struct Arp<const NUM: usize> {
    pub map: [Option<(layer3::ipv4::Address, layer2::Address)>; NUM],
}

impl<const NUM: usize> Default for Arp<NUM> {
    fn default() -> Self {
        Self { map: [None; NUM] }
    }
}

impl<const NUM: usize> ArpStorage for Arp<NUM> {
    fn set_map(&mut self, mac: layer2::Address, ip_addr: layer3::ipv4::Address) -> Result<()> {
        if let Ok(pos) = self.map.binary_search(&None) {
            self.map[pos] = Some((ip_addr, mac));
            Ok(())
        } else {
            Err(Error::NoSpaceForArpStorage)
        }
    }

    fn mac_addr(&self, ip_addr: &layer3::ipv4::Address) -> Result<Option<layer2::Address>> {
        if let Ok(pos) = self
            .map
            .binary_search_by_key(&Some(*ip_addr), |a| a.map(|(a, _)| a))
        {
            if let Some((_, b)) = self.map[pos] {
                Ok(Some(b))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}
