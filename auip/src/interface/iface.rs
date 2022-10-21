use auip_pkt::{
    layer2::{self, ethernet, VlanId},
    layer3,
};

use crate::{interface::utils, AddrsStorage, ArpStorage, Device, Medium, Result};

use super::action::Action;

pub struct Interface<D, AS, ARPS> {
    device: D,
    medium: Medium,

    // Vland ID
    vlanid0: Option<VlanId>,
    vlanid1: Option<VlanId>,

    // Address storage
    addrs_storage: AS,

    arp_storage: ARPS,
}

impl<D, AS, ARPS> Interface<D, AS, ARPS>
where
    D: Device,
    AS: AddrsStorage,
    ARPS: ArpStorage,
{
    pub fn new(device: D, addrs_storage: AS, arp_storage: ARPS) -> Self {
        let medium = device.medium();

        Self {
            device,
            medium,
            addrs_storage,
            vlanid0: None,
            vlanid1: None,
            arp_storage,
        }
    }

    pub fn device(&self) -> &D {
        &self.device
    }

    pub fn device_mut(&mut self) -> &mut D {
        &mut self.device
    }

    pub fn addrs_storage(&self) -> &AS {
        &self.addrs_storage
    }

    pub fn addrs_storage_mut(&mut self) -> &mut AS {
        &mut self.addrs_storage
    }

    pub fn arp_storage(&self) -> &ARPS {
        &self.arp_storage
    }

    pub fn arp_storage_mut(&mut self) -> &mut ARPS {
        &mut self.arp_storage
    }

    pub(crate) fn poll_ethernet(&mut self) -> Result<Action> {
        let device = &mut self.device;

        let this_mac_addr = *self.addrs_storage.mac_addr();

        let arp_storage = &mut self.arp_storage;

        let addrs_storage = &mut self.addrs_storage;

        if let Some(rx_bytes) = device.recv()? {
            let mut rx_pkt = ethernet::Packet::new_checked(rx_bytes)?;

            log::debug!("Receive ethernet packet: {}", rx_pkt);

            let dest_addr = rx_pkt.dest_addr();

            if dest_addr != this_mac_addr && dest_addr != layer2::Address::BROADCAST {
                log::debug!("Mac address mismatch, Drop it.");

                return Ok(Action::NoAction);
            }

            let protocol = rx_pkt.protocol();

            let l3 = match protocol {
                layer2::Protocol::Layer3Protocol(l3) => l3,
                layer2::Protocol::IEEE8021Q(vlanid, l3) => {
                    if Some(vlanid) == self.vlanid0 {
                        l3
                    } else {
                        log::debug!("VlanId mismatch, Drop it.");
                        return Ok(Action::NoAction);
                    }
                }
                layer2::Protocol::QinQ(vlanid, vlanid1, l3) => {
                    if Some(vlanid) == self.vlanid0 && Some(vlanid1) == self.vlanid1 {
                        l3
                    } else {
                        log::debug!("VlanId mismatch, Drop it.");
                        return Ok(Action::NoAction);
                    }
                }

                // TODO: process IEEE802.3 packet.
                layer2::Protocol::Length(_) => {
                    log::debug!("Unsupport IEEE802.3. This format will support later, Drop it.");
                    return Ok(Action::NoAction)
                }

                // Skip
                layer2::Protocol::Unknown(ty) => {
                    log::debug!("Unsupport protocol type: {}, Drop it.", ty);
                    return Ok(Action::NoAction);
                }
            };

            match l3 {
                layer2::Layer3Protocol::ARP => {
                    let pkt = layer3::arp::Packet::new_checked(rx_pkt.payload_mut())?;

                    log::debug!("Receive arp packet: {}", pkt);

                    let sha = pkt.source_hardware_address()?;
                    let spa = pkt.source_protocol_address()?;
                    let tpa = pkt.target_protocol_address()?;
                    let sa = rx_pkt.src_addr();

                    return utils::build_and_record_arp(
                        sa,
                        sha,
                        spa,
                        this_mac_addr,
                        tpa,
                        addrs_storage,
                        arp_storage,
                    );
                }
                layer2::Layer3Protocol::IPv4 => {
                    let pkt = layer3::ipv4::Packet::new_checked(rx_pkt.payload_mut())?;

                    utils::poll_ipv4(pkt)?;
                }
                layer2::Layer3Protocol::IPv6 => {}
                layer2::Layer3Protocol::Unknown(_) => {}
            }
        }

        Ok(Action::NoAction)
    }

    pub(crate) fn poll_ip(&mut self) -> Result<()> {
        if let Some(rx_bytes) = self.device.recv()? {
            let ip_pkt = layer3::IpPacket::parse(rx_bytes)?;

            match ip_pkt {
                layer3::IpPacket::IPv4(pkt) => utils::poll_ipv4(pkt)?,
                layer3::IpPacket::Ipv6 => {}
            }
        }

        Ok(())
    }

    pub fn poll(&mut self) -> Result<()> {
        match self.medium {
            Medium::Ethernet => match self.poll_ethernet()? {
                Action::NoAction => {}
                Action::SendArp(bytes) => self.device.send(&bytes)?,
            },
            Medium::Ip => self.poll_ip()?,
        }

        Ok(())
    }
}
