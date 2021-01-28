//! interface layer.

// use crate::phy::Device;
use auip_pkt::{ip, mac};
use managed::ManagedSlice;

pub struct Interface<'a> {
    name: &'a str,
    // device: &'a DeviceT,
    // ip_address: ManagedSlice<'a, ip::Address>,
    // mac_address: mac::Address,
}

impl<'a> Interface<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            // device,
            // address: ManagedSlice::Borrowed(&mut[])
        }
    }

    pub async fn loop_arp(&mut self) {

    }

    pub async fn loop_icmp(&mut self) {

    }
}
