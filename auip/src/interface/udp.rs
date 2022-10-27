use auip_pkt::layer4;

use crate::Result;

pub fn poll_udp(pkt: layer4::udp::Packet<&[u8]>) -> Result<()> {
    log::debug!("Receive packet: {}", pkt);
    Ok(())
}
