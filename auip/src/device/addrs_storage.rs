use auip_pkt::{ip, mac};

use crate::Result;

pub trait AddrsStorage {
    fn set_mac_addr(&mut self, addr: mac::Address);

    fn mac_addr(&self) -> &mac::Address;

    fn add_ip_addr(&mut self, addr: ip::Cidr) -> Result<()>;

    fn del_ip_addr(&mut self, addr: ip::Cidr);

    fn ip_addrs(&self) -> &[ip::Cidr];
}
