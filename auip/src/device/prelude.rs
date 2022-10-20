use auip_pkt::{layer2, layer3};

use crate::{Medium, Result};

pub trait Device {
    fn send(&mut self, buffer: &[u8]) -> Result<()>;

    fn recv(&mut self) -> Result<Option<&[u8]>>;

    fn medium(&self) -> Medium;
}

pub trait AddrsStorage {
    fn set_mac_addr(&mut self, addr: layer2::Address);

    fn mac_addr(&self) -> &layer2::Address;

    fn add_ip_addr(&mut self, addr: layer3::Cidr) -> Result<()>;

    fn del_ip_addr(&mut self, addr: layer3::Cidr);

    fn ip_addrs(&self) -> &[layer3::Cidr];

    fn has_ip_addr(&self, ip_addr: &layer3::Address) -> bool {
        let addrs = self.ip_addrs();

        for cidr in addrs {
            if cidr.contains_addr(ip_addr) {
                return true;
            }
        }

        false
    }
}

pub trait ArpStorage {
    fn set_map(&mut self, mac: layer2::Address, ip_addr: layer3::ipv4::Address) -> Result<()>;

    fn mac_addr(&self, ip_addr: layer3::ipv4::Address) -> Result<layer2::Address>;
}

/* pub trait Layer3PacketStorage { */
/* type Layer3PacketBytes; */
/*  */
/* fn get(&self, idx: usize) -> Option<&layer3::Packet<Self::Layer3PacketBytes>>; */
/*  */
/* fn get_mut(&mut self, idx: usize) -> Option<&mut layer3::Packet<Self::Layer3PacketBytes>>; */
/*  */
/* fn length(&self) -> usize; */
/* } */
