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
    if pkt.dont_frag() {
        // enter upper layer.
        let protocol = pkt.protocol();

        let payload = pkt.payload();

        match protocol {
            layer3::Protocol::Udp => {
                let pkt = layer4::udp::Packet::new_checked(payload)?;

                poll_udp(pkt)?;
            }
            _ => {}
        }
    } else {
        let ident = pkt.ident();

        // Ip fragment support.
        if pkt.more_frags() {
            // save frag.
            let buffer = ip_fragment_buffer.get_buffer(ident);
            let offset = pkt.frag_offset();
        } else {
            // complete frag.
        }
    }

    Ok(())
}
