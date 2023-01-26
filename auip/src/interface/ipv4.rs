use auip_pkt::layer3::{
    ipv4::{field, Packet},
    Protocol,
};

use crate::{build_time_exceeded_ttl, poll_icmpv4, poll_udp, IpFragmentBuffer, Result, bytes::Icmpv4Bytes};

pub(crate) fn poll_ipv4(
    pkt: Packet<&[u8]>,
    ip_fragment_buffer: &mut impl IpFragmentBuffer,
) -> Result<()> {
    log::debug!("Receive packet: {}", pkt);

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

            let buffer = ip_fragment_buffer.get_buffer(ident);
            let target_buf = &mut buffer[offset..payload_len];
            target_buf.copy_from_slice(payload);

            return Ok(());
        } else {
            let length = pkt.total_len() - pkt.header_len() as u16 + pkt.frag_offset();
            let buffer = ip_fragment_buffer.get_buffer(ident);
            &buffer[0..length as usize]
        }
    };

    let protocol = pkt.protocol();

    match protocol {
        Protocol::Udp => {
            poll_udp(payload)?;
        }
        Protocol::Icmp => {
            poll_icmpv4(payload)?;
        }
        _ => {}
    }

    Ok(())
}

pub(crate) fn build_response_packet<SendInner>(
    recv: &Packet<&[u8]>,
    send: &mut Packet<SendInner>,
    protocol: Protocol,
) where
    SendInner: AsRef<[u8]> + AsMut<[u8]>,
{
    send.set_version(recv.version());
    send.set_header_len(field::HEADER_LEN_WITHOUT_OPTION);
    send.set_ident(recv.ident());
    send.set_dont_frag(true);
    send.set_ttl(64);
    send.set_protocol(protocol);
    send.set_src_addr(recv.dst_addr());
    send.set_dst_addr(recv.src_addr());
}

pub(crate) fn build_icmp_time_expire_ttl_packet(recv: &Packet<&[u8]>) {
    let bytes = Icmpv4Bytes::default();

    let mut send = Packet::new_unchecked(bytes);

    build_response_packet(recv, &mut send, Protocol::Icmp);
    let payload = send.payload_mut();

    build_time_exceeded_ttl(payload);

    send.fill_checksum();
}


