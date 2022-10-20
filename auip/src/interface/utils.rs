use auip_pkt::layer3;

use crate::{ArpStorage, Result};

pub fn poll_arp(pkt: layer3::arp::Packet<&[u8]>, arps: &mut impl ArpStorage) -> Result<()> {
    let l3_addr = pkt.target_protocol_address()?;

    if let layer3::arp::ProtocolAddress::IPv4(addr) = l3_addr {}

    Ok(())
}

pub fn poll_ipv4(bytes: layer3::ipv4::Packet<&[u8]>) -> Result<()> {
    Ok(())
}
