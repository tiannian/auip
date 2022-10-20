use core::mem;

use auip_pkt::{
    layer2::{self, ethernet, VlanId},
    layer3,
};

use crate::{AddrsStorage, Device, Layer3PacketStorage, Medium, Result, RingCurser};

pub struct Interface<D, AS, L3PS> {
    device: D,
    medium: Medium,

    // Vland ID
    vlanid0: Option<VlanId>,
    vlanid1: Option<VlanId>,

    // Address storage
    addrs_storage: AS,

    // layer3 packet
    layer3_packet_storage: L3PS,
    l3ps_ring_curser: RingCurser,
}

impl<D, AS, L3PS> Interface<D, AS, L3PS>
where
    D: Device,
    AS: AddrsStorage,
    L3PS: Layer3PacketStorage,
{
    pub fn new(device: D, addrs_storage: AS, layer3_packet_storage: L3PS) -> Self {
        let medium = device.medium();

        let l3ps_ring_curser = RingCurser::new(layer3_packet_storage.length());

        Self {
            device,
            medium,
            addrs_storage,
            layer3_packet_storage,
            vlanid0: None,
            vlanid1: None,
            l3ps_ring_curser,
        }
    }

    pub(crate) fn poll_ethernet(&mut self) -> Result<()> {
        if let Some(rx_bytes) = self.device.recv()? {
            let rx_pkt = ethernet::Packet::new_checked(rx_bytes)?;

            let dest_addr = rx_pkt.dest_addr();

            if &dest_addr == self.addrs_storage.mac_addr() {
                let protocol = rx_pkt.protocol();

                match protocol {
                    layer2::Protocol::Layer3Protocol(l3) => poll_layer3(l3, rx_pkt.payload())?,
                    layer2::Protocol::IEEE8021Q(vlanid, l3) => {
                        if Some(vlanid) == self.vlanid0 {
                            poll_layer3(l3, rx_pkt.payload())?;
                        }
                    }
                    layer2::Protocol::QinQ(vlanid, vlanid1, l3) => {
                        if Some(vlanid) == self.vlanid0 && Some(vlanid1) == self.vlanid1 {
                            poll_layer3(l3, rx_pkt.payload())?;
                        }
                    }

                    // TODO: process IEEE802.3 packet.
                    layer2::Protocol::Length(_) => {}

                    // Skip
                    layer2::Protocol::Unknown(_) => {}
                }
            }

            // Select mac address and send packet
        }

        Ok(())
    }

    pub(crate) fn poll_ip(&mut self) -> Result<()> {
        if let Some(rx_bytes) = self.device.recv()? {
            let ip_pkt = layer3::IpPacket::parse(rx_bytes)?;

            match ip_pkt {
                layer3::IpPacket::IPv4(pkt) => poll_ipv4(pkt)?,
                layer3::IpPacket::Ipv6 => {}
            }
        }

        Ok(())
    }

    pub(crate) fn poll_send(&mut self) -> Result<()> {
        if !self.l3ps_ring_curser.is_empty() {
            while let Some(idx) = self.l3ps_ring_curser.pop() {
                if let Some(pkt) = self.layer3_packet_storage.get_mut(idx) {
                    let _pkt = mem::replace(pkt, layer3::Packet::Unspecified);

                    // send pkt here
                }
            }
        }

        Ok(())
    }

    pub fn poll(&mut self) -> Result<()> {
        self.poll_send()?;
        
        match self.medium {
            Medium::Ethernet => self.poll_ethernet()?,
            Medium::Ip => self.poll_ip()?,
        }

        Ok(())
    }
}

pub(crate) fn poll_layer3(protocol: layer2::Layer3Protocol, bytes: &[u8]) -> Result<()> {
    match protocol {
        layer2::Layer3Protocol::IPv4 => {
            let pkt = layer3::ipv4::Packet::new_checked(bytes)?;

            poll_ipv4(pkt)?;
        }
        layer2::Layer3Protocol::IPv6 => {}
        layer2::Layer3Protocol::ARP => {}
        layer2::Layer3Protocol::Unknown(_) => {}
    }

    Ok(())
}

pub(crate) fn poll_arp(bytes: &[u8]) -> Result<()> {
    let pkt = layer3::arp::Packet::new_checked(bytes)?;

    Ok(())
}

pub(crate) fn poll_ipv4(pkt: layer3::ipv4::Packet<&[u8]>) -> Result<()> {
    Ok(())
}
