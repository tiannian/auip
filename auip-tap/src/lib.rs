#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
mod error;
pub use error::{Error, Result};

mod tap;
pub use tap::open_tap_device;

use auip::phy::DeviceCapabilities;
use auip_pkt::mac::ethernet::Packet;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

pub struct TapDevice {
    // tx_buffer: [u8; 1536],
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

// impl auip::phy::Device for TapDevice {
//     type ReturnReceiveFuture<'__async_trait> =
//         impl core::future::Future<Output = &'__async_trait mut [u8]>;
//     fn receive<'__async_trait>(
//         &'__async_trait mut self,
//     ) -> Self::ReturnReceiveFuture<'__async_trait> {
//         async move {
//             let size = self.fs.read(&mut self.rx_buffer).await.unwrap();
//             println!("Receive packet length: {}", size);
//             let pkt = Packet::new_checked(&self.rx_buffer[..size]).unwrap();
//             println!("{}", pkt);
//             self.rx_buffer.as_mut()
//         }
//     }

//     fn capabilities(&self) -> DeviceCapabilities {
//         DeviceCapabilities {
//             max_transmission_unit: 1536,
//             max_burst_size: Some(1536),
//             layer: auip::phy::DeviceLayer::Mac,
//         }
//     }
// }
