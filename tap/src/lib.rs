#![allow(incomplete_features)]
#![feature(generic_associated_types)]
mod error;
use auip::Device;
pub use error::{Error, Result};

mod tap;
pub use tap::open_tap_device;

use tokio::fs::File;

pub struct TapDevice {
    rx_buffer: [u8; 1536],
    fs: File,
}

impl TapDevice {
    pub fn new(fs: File) -> Self {
        Self {
            rx_buffer: [0u8; 1536],
            fs,
        }
    }
}

impl Device for TapDevice {
    fn medium(&self) -> auip::Medium {
        auip::Medium::Ethernet
    }

    fn recv(&mut self) -> auip::Result<Option<&[u8]>> {
        Ok(None)
    }

    fn send(&mut self, buffer: &[u8]) -> auip::Result<()> {
        Ok(())
    }
}

/* impl auip::phy::Driver for TapDevice { */
    /* type ReturnReceiveFuture<'__async_trait> = */
    /*     impl core::future::Future<Output = Option<mac::Packet<&'__async_trait [u8]>>>; */
    /*  */
    /* fn receive(&mut self) -> Self::ReturnReceiveFuture<'_> { */
    /*     async move { */
    /*         let size = self.fs.read(&mut self.rx_buffer).await.unwrap(); */
    /*         let buffer = self.rx_buffer[..size].as_ref(); */
    /*         let pkt = mac::Packet::EthernetII(mac::ethernet::Packet::new_checked(buffer).unwrap()); */
    /*         Some(pkt) */
    /*     } */
    /* } */
    /*  */
    /* fn capabilities(&self) -> DeviceCapabilities { */
    /*     DeviceCapabilities { */
    /*         max_transmission_unit: 1536, */
    /*         max_burst_size: Some(1536), */
    /*         layer: auip::phy::DeviceLayer::Layer2, */
    /*     } */
    /* } */
/* } */
