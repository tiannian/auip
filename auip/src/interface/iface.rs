use auip_pkt::{
    layer2::{self, ethernet, VlanId},
    layer3,
};

use crate::{interface::utils, AddrsStorage, ArpStorage, Device, Error, Medium, Result};

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

    pub(crate) fn poll_ethernet(&mut self) -> Result<()> {
        let device = &mut self.device;

        let this_mac_addrs = *self.addrs_storage.mac_addr();

        let arp_storage = &mut self.arp_storage;

        let addrs_storage = &mut self.addrs_storage;

        if let Some(rx_bytes) = device.recv()? {
            let rx_pkt = ethernet::Packet::new_checked(rx_bytes)?;

            log::debug!("Receive ethernet packet: {}", rx_pkt);

            let dest_addr = rx_pkt.dest_addr();

            if dest_addr != this_mac_addrs {
                log::debug!("Mac address mismatch, Drop it.");
            }
            let protocol = rx_pkt.protocol();

            let l3 = match protocol {
                layer2::Protocol::Layer3Protocol(l3) => l3,
                layer2::Protocol::IEEE8021Q(vlanid, l3) => {
                    if Some(vlanid) == self.vlanid0 {
                        l3
                    } else {
                        log::debug!("VlanId mismatch, Drop it.");
                        return Ok(());
                    }
                }
                layer2::Protocol::QinQ(vlanid, vlanid1, l3) => {
                    if Some(vlanid) == self.vlanid0 && Some(vlanid1) == self.vlanid1 {
                        l3
                    } else {
                        log::debug!("VlanId mismatch, Drop it.");
                        return Ok(());
                    }
                }

                // TODO: process IEEE802.3 packet.
                layer2::Protocol::Length(_) => return Ok(()),

                // Skip
                layer2::Protocol::Unknown(ty) => {
                    log::debug!("Unsupport protocol type: {}, Drop it.", ty);
                    return Ok(());
                }
            };

            match l3 {
                layer2::Layer3Protocol::ARP => {
                    let pkt = layer3::arp::Packet::new_checked(rx_pkt.payload())?;

                    {
                        log::debug!("Receive arp packet: {}", pkt);

                        let mac_addr = pkt
                            .source_hardware_address()?
                            .mac_addr()
                            .ok_or(Error::UnexpectedType)?;
                        let ip_addr = pkt
                            .source_protocol_address()?
                            .ipv4_addr()
                            .ok_or(Error::UnexpectedType)?;

                        log::debug!("add arp map {} -> {}", &mac_addr, &ip_addr);

                        arp_storage.set_map(mac_addr, ip_addr)?;
                    }

                    let addr = pkt
                        .target_protocol_address()?
                        .ipv4_addr()
                        .ok_or(Error::UnexpectedType)?;

                    let ip_addr = layer3::Address::Ipv4(addr);

                    if addrs_storage.has_ip_addr(&ip_addr) {
                        let _rpkt = utils::poll_arp(pkt, arp_storage)?;
                    } else {
                        log::debug!("Ip address mismatch, Drop it.");
                    }
                }
                layer2::Layer3Protocol::IPv4 => {
                    let pkt = layer3::ipv4::Packet::new_checked(rx_pkt.payload())?;

                    utils::poll_ipv4(pkt)?;
                }
                layer2::Layer3Protocol::IPv6 => {}
                layer2::Layer3Protocol::Unknown(_) => {}
            }
        }

        Ok(())
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
            Medium::Ethernet => self.poll_ethernet()?,
            Medium::Ip => self.poll_ip()?,
        }

        Ok(())
    }
}
