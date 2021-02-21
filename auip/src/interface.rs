//! interface layer.

use crate::phy::{DevicePoll, Device, Driver};
// use auip_pkt::{ip, mac};
// use managed::ManagedSlice;

pub struct Interface<'a> {
    pub name: &'a str,
    pub device: &'a mut dyn DevicePoll,
    // ip_address: ManagedSlice<'a, ip::Address>,
    // mac_address: mac::Address,
}

impl<'a> Interface<'a> {
    pub fn new<D: Driver>(name: &'a str, device: &'a mut Device<D>) -> Self {
        Self {
            name,
            device,
            // address: ManagedSlice::Borrowed(&mut[])
        }
    }

    pub async fn loop_arp(&mut self) {

    }

    pub async fn loop_icmp(&mut self) {

    }
}
