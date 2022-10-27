use auip_pkt::{layer3, layer4};

use crate::{Result, poll_udp};

pub fn poll_ipv4(pkt: layer3::ipv4::Packet<&[u8]>) -> Result<()> {
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
        return Ok(());
    }

    if pkt.more_frags() {
        // save frag.
    } else {
        // complete frag.
    }

    Ok(())
}
