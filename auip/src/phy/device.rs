use super::driver::Driver;
use managed::ManagedMap;
// use super::driver::DriverReceivePacket;
use crate::interface::Interface;
use auip_pkt::mac;

pub struct Device<'a> {
    interfaces: ManagedMap<'a, mac::Address, &'a mut Interface<'a>>,
}

impl<'a> Device<'a> {
    pub fn new<const N: usize>(interfaces: &mut [Interface<'a>; N]) -> Self {
        // let 
        Self {
            interfaces: ManagedMap::Borrowed(&mut [])
        }
    }

    pub async fn receive<D: Driver>(&mut self, driver: &mut D) {
        // let (layer2_pkt, layer3_pkt) = driver.receive().await;
        // if let Some(repr) = layer2_pkt {
        //     if let Some(target) = self.interfaces.get(&repr.destination) {
        //         // send packet to target.
        //     } else {
        //         // send all
        //     }
        // } else {
        //     // send all
        // }
    }
}
