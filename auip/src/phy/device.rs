use super::driver::Driver;
// use managed::ManagedMap;
// use super::driver::DriverReceivePacket;
// use crate::interface::Interface;
use auip_pkt::ip;
use auip_pkt::mac;
use crate::{Error, Result};
use core::{task, pin, future::Future};

pub struct Device<D: Driver> {
    driver: D,
}

pub trait DevicePoll {
    fn poll_receive(&mut self, cx: &mut task::Context) -> task::Poll<Result<ip::Packet<&[u8]>>>;
}

impl<D: Driver> DevicePoll for Device<D> {
    fn poll_receive(&mut self, cx: &mut task::Context) -> task::Poll<Result<ip::Packet<&[u8]>>> {
        let mut fu = self.receive();
        unsafe { pin::Pin::new_unchecked(&mut fu).poll(cx) }
    }
}

impl<D: Driver> Device<D> {
    pub fn new(driver: D) -> Self {
        // let
        Self {
            driver,
        }
    }

    pub async fn receive(&mut self) -> Result<ip::Packet<&[u8]>> {
        if let Some(layer2_pkt) = self.driver.receive().await {
            log::debug!("Receive layer2 packet \n{}", layer2_pkt);
            let buffer = layer2_pkt.payload().unwrap();
            match layer2_pkt.protocol() {
                mac::Protocol::ARP => {
                    let arp = ip::arp::Packet::new_checked(buffer).unwrap();
                    log::debug!("Receive layer3 packet \n{}", arp);
                    Ok(ip::Packet::ARP(arp))
                }
                _ => Ok(ip::Packet::IPv6)
            }
        } else {
            Err(Error::DriverPacketError)
        }
    }
}
