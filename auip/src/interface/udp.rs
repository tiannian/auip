use auip_pkt::layer4::udp::Packet;

use crate::Result;

pub(crate) fn poll_udp(bytes: &[u8]) -> Result<()> {
    let pkt = Packet::new_checked(bytes)?;

    log::debug!("Receive packet: {}", pkt);
    Ok(())
}
