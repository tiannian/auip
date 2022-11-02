use auip_pkt::{layer3, layer4};

use crate::{poll_udp, IpFragmentBuffer, Result};

pub fn poll_ipv4(
    pkt: layer3::ipv4::Packet<&[u8]>,
    ip_fragment_buffer: &mut impl IpFragmentBuffer,
) -> Result<()> {
    log::debug!("Receive packet: {}", pkt);

    // Drop ttl is 0.
    if pkt.ttl() == 0 {
        return Ok(());
    }

    // Check is fragment
    let payload = if pkt.dont_frag() {
        // enter upper layer.
        pkt.payload()
    } else {
        let ident = pkt.ident();

        // Ip fragment support.
        if pkt.more_frags() {
            // save frag.
            let payload = pkt.payload();
            let payload_len = payload.len();
            let offset = pkt.frag_offset() as usize;

            if let Some(buffer) = ip_fragment_buffer.get_buffer(ident) {
                let target_buf = &mut buffer[offset..payload_len];

                target_buf.copy_from_slice(payload);
            } else {
                log::warn!("No buffer to store ip fragment");
            }

            return Ok(());
        } else {
            let length = pkt.total_len();

            if let Some(buffer) = ip_fragment_buffer.get_buffer(ident) {
                &buffer[0..length as usize]
            } else {
                log::warn!("No buffer to store ip fragment");
                return Ok(());
            }
        }
    };

    let protocol = pkt.protocol();

    match protocol {
        layer3::Protocol::Udp => {
            let pkt = layer4::udp::Packet::new_checked(payload)?;

            poll_udp(pkt)?;
        }
        _ => {}
    }

    Ok(())
}
