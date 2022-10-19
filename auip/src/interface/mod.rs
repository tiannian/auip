use auip_pkt::{
    ip,
    mac::{ethernet, VlanId},
};

use crate::{AddrsStorage, Device, Medium, Result};

pub struct Interface<D, A> {
    device: D,
    medium: Medium,
    addrs_storage: A,
    vlanid: VlanId,
}

impl<D: Device, A: AddrsStorage> Interface<D, A> {
    pub fn new(device: D, addrs_storage: A, vlanid: VlanId) -> Self {
        let medium = device.medium();

        Self {
            device,
            medium,
            addrs_storage,
            vlanid,
        }
    }

    pub(crate) fn poll_ethernet(&mut self) -> Result<()> {
        let rx_bytes = self.device.recv()?;
        let rx_pkt = ethernet::Packet::new_checked(rx_bytes)?;

        // TODO: Hook

        let dest_addr = rx_pkt.dest_addr();

        if &dest_addr == self.addrs_storage.mac_addr() {
            // Got vlan id and math
            // Process ip packet
        }

        // Select mac address and send packet

        Ok(())
    }

    pub(crate) fn poll_ip(&mut self) -> Result<()> {
        let rx_bytes = self.device.recv()?;
        // let rx_pkt = ip::Packet::new_checked(rx_bytes)?;

        Ok(())
    }

    pub fn poll(&mut self) -> Result<()> {
        match self.medium {
            Medium::Ethernet => self.poll_ethernet()?,
            Medium::Ip => self.poll_ip()?,
        }

        Ok(())
    }
}
