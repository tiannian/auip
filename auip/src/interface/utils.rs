use auip_pkt::{layer2, layer3, IntoInner};

use crate::{interface::action::ArpBytes, AddrsStorage, ArpStorage, Error, Result};

use super::action::Action;

pub fn build_and_record_arp(
    sa: layer2::Address,
    sha: layer3::arp::HardwareAddress,
    spa: layer3::arp::ProtocolAddress,
    tha: layer2::Address,
    tpa: layer3::arp::ProtocolAddress,
    addrs_storage: &impl AddrsStorage,
    arp_storage: &mut impl ArpStorage,
) -> Result<Action> {
    let mac_addr = sha.mac_addr().ok_or(Error::UnexpectedType)?;
    let ip_addr = spa.ipv4_addr().ok_or(Error::UnexpectedType)?;

    log::debug!("add arp map {} -> {}", &mac_addr, &ip_addr);

    arp_storage.set_map(*mac_addr, *ip_addr)?;

    let addr = tpa.ipv4_addr().ok_or(Error::UnexpectedType)?;

    let ip_addr = layer3::Address::Ipv4(*addr);

    if addrs_storage.has_ip_addr(&ip_addr) {
        let arp_bytes = ArpBytes::default();

        let mut layer2_pkt = layer2::ethernet::Packet::new_unchecked(arp_bytes);

        let mut pkt = layer3::arp::Packet::new_unchecked(layer2_pkt.payload_mut());

        pkt.set_operation(layer3::arp::Operation::Reply);

        pkt.set_source_hardware_address(layer3::arp::HardwareAddress::Ethernet(tha))?;
        pkt.set_source_protocol_address(tpa)?;
        pkt.set_target_hardware_address(sha)?;
        pkt.set_target_protocol_address(spa)?;

        layer2_pkt.set_src_addr(*addrs_storage.mac_addr());
        layer2_pkt.set_dest_addr(sa);
        layer2_pkt.set_protocol(layer2::Protocol::Layer3Protocol(
            layer2::Layer3Protocol::ARP,
        ));

        log::debug!("Send packet: {}", layer2_pkt);

        return Ok(Action::SendArp(layer2_pkt.into_inner()));
    } else {
        log::debug!("Ip address mismatch, Drop it.");
        return Ok(Action::NoAction);
    }
}

pub fn poll_ipv4(bytes: layer3::ipv4::Packet<&mut [u8]>) -> Result<()> {
    Ok(())
}
