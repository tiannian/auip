use std::{
    fs::File,
    io::{Read, Write},
};

use auip::Device;

use crate::{open_device, Result};

pub struct TapTunDevice {
    pub rx_buffer: [u8; 1536],
    pub len: usize,
    pub file: File,
}

impl TapTunDevice {
    pub fn new_tap(ifname: &str) -> Result<Self> {
        let file = open_device(ifname, true)?;

        Ok(Self {
            rx_buffer: [0u8; 1536],
            len: 0,
            file,
        })
    }

    pub fn new_tun(ifname: &str) -> Result<Self> {
        let file = open_device(ifname, false)?;

        Ok(Self {
            rx_buffer: [0u8; 1536],
            len: 0,
            file,
        })
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
            Ok(Some(&self.rx_buffer[..self.len]))
        }
    }

    fn send(&mut self, buffer: &[u8]) -> auip::Result<()> {
        self.file.write_all(buffer).unwrap();
        Ok(())
    }
}
