//! interface layer.

use crate::phy::Device;
use auip_pkt::ip;
use managed::ManagedSlice;

pub struct Interface<'a, DeviceT: Device> {
    name: &'a str,
    device: &'a DeviceT,
    address: ManagedSlice<'a, ip::Address>,
}

impl<'a, DeviceT: Device> Interface<'a, DeviceT> {
    pub fn new(name: &'a str, device: &'a DeviceT) -> Self {
        Self {
            name,
            device,
            address: ManagedSlice::Borrowed(&mut[])
        }
    }
}
