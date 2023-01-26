use auip_pkt::{layer2, layer3, IntoInner};

use crate::{bytes::ArpBytes, AddrsStorage, ArpStorage, Error, InterfaceConfig, Result};

#[allow(clippy::too_many_arguments)]
pub(crate) fn build_and_record_arp(
    sa: layer2::Address,
    sha: layer3::arp::HardwareAddress,
    spa: layer3::arp::ProtocolAddress,
    tha: layer2::Address,
    tpa: layer3::arp::ProtocolAddress,
    config: &InterfaceConfig,
    addrs_storage: &impl AddrsStorage,
    arp_storage: &mut impl ArpStorage,
) -> Result<Option<ArpBytes>> {
    let mac_addr = sha.mac_addr().ok_or(Error::UnexpectedType)?;
    let ip_addr = spa.ipv4_addr().ok_or(Error::UnexpectedType)?;

    log::debug!("add arp map {} -> {}", &mac_addr, &ip_addr);

    arp_storage.set_map(*mac_addr, *ip_addr)?;

    let addr = tpa.ipv4_addr().ok_or(Error::UnexpectedType)?;

    let ip_addr = layer3::Address::Ipv4(*addr);

    if addrs_storage.has_ip_addr(&ip_addr) {
        let arp_bytes = ArpBytes::default();

        let mut layer2_pkt = layer2::ethernet::Packet::new_unchecked(arp_bytes);

        layer2_pkt.set_src_addr(*addrs_storage.mac_addr());
        layer2_pkt.set_dest_addr(sa);

        let layer3_protocol = layer2::Layer3Protocol::ARP;

        if config.vlan.tag_vlan0 {
            let vlanid = config.vlan.vlanid0.ok_or(Error::NoVlanIdSet)?;
            layer2_pkt.set_protocol(layer2::Protocol::IEEE8021Q(vlanid, layer3_protocol));
        } else if config.vlan.tag_vlan0 && config.vlan.tag_vlan1 {
            let vlanid0 = config.vlan.vlanid0.ok_or(Error::NoVlanIdSet)?;
            let vlanid1 = config.vlan.vlanid0.ok_or(Error::NoVlanIdSet)?;
            layer2_pkt.set_protocol(layer2::Protocol::QinQ(vlanid0, vlanid1, layer3_protocol));
        } else {
            layer2_pkt.set_protocol(layer2::Protocol::Layer3Protocol(layer3_protocol));
        }

        let mut pkt = layer3::arp::Packet::new_unchecked(layer2_pkt.payload_mut());

        pkt.set_operation(layer3::arp::Operation::Reply);

        pkt.set_source_hardware_address(layer3::arp::HardwareAddress::Ethernet(tha))?;
        pkt.set_source_protocol_address(tpa)?;
        pkt.set_target_hardware_address(sha)?;
        pkt.set_target_protocol_address(spa)?;

        log::debug!("Send packet: {}", layer2_pkt);

        Ok(Some(layer2_pkt.into_inner()))
    } else {
        log::debug!("Ip address mismatch, Drop it.");
        Ok(None)
    }
}
