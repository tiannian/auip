mod error;
use std::{fs::File, io::{Write, Read}};

use auip::Device;
pub use error::{Error, Result};

mod tap;
pub use tap::open_tap_device;

pub struct TapTunDevice {
    pub rx_buffer: [u8; 1536],
    pub len: usize,
    pub file: File,
}

impl TapTunDevice {
    pub fn new(file: File) -> Self {
        Self {
            rx_buffer: [0u8; 1536],
            len: 0,
            file
        }
    }

    pub fn poll_read(&mut self) {
        let len = self.file.read(&mut self.rx_buffer).unwrap();
        self.len = len;
    }
}

impl Device for TapTunDevice {
    fn medium(&self) -> auip::Medium {
        auip::Medium::Ethernet
    }

    fn recv(&mut self) -> auip::Result<Option<&[u8]>> {
        if self.len == 0 {
            Ok(None)
        } else {
            Ok(Some(&self.rx_buffer[.. self.len]))
        }
    }

    fn send(&mut self, buffer: &[u8]) -> auip::Result<()> {
        self.file.write_all(buffer).unwrap();
        Ok(())
    }
}

