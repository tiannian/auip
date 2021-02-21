//! interface layer.

use crate::phy::{Device, DevicePoll, Driver};
use auip_pkt::{ip, mac};
use managed::ManagedSlice;

pub struct Interface<'a> {
    name: &'a str,
    device: &'a mut dyn DevicePoll,
    ip_address: ManagedSlice<'a, ip::Cidr>,
    mac_address: mac::Address,
}

impl<'a> Interface<'a> {
    pub fn new<D: Driver>(
        name: &'a str,
        device: &'a mut Device<D>,
        mac_address: mac::Address,
    ) -> Self {
        Self {
            name,
            device,
            ip_address: ManagedSlice::Borrowed(&mut []),
            mac_address,
        }
    }

    /// get interface name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// rename interface's name.
    pub fn rename(&mut self, name: &'a str) {
        self.name = name
    }

    /// get interface's mac address.
    pub fn mac_address(&self) -> &mac::Address {
        &self.mac_address
    }

    /// get interface's ip layer address in cidr format.
    pub fn ip_address(&self) -> &[ip::Cidr] {
        &self.ip_address
    }

    /// update interface's ip layer address.
    pub fn update_address<F>(&mut self, f: F)
    where
        F: FnOnce(&mut ManagedSlice<'a, ip::Cidr>),
    {
        f(&mut self.ip_address)
    }

    pub async fn loop_arp(&mut self) {}

    pub async fn loop_icmp(&mut self) {}
}
