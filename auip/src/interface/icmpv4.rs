use auip_pkt::layer4::icmpv4::{Message, Packet, TimeExceeded};

use crate::Result;

pub fn poll_icmpv4(bytes: &[u8]) -> Result<()> {
    let pkt = Packet::new_checked(bytes)?;

    log::debug!("Receive packet: {}", pkt);
    Ok(())
}

fn build_time_exceeded(te: TimeExceeded, bytes: &mut [u8]) {
    let mut pkt = Packet::new_unchecked(bytes);

    let protocol = Message::TimeExceeded(te);

    pkt.set_protocol(protocol);
    pkt.fill_checksum();
}

pub fn build_time_exceeded_ttl(bytes: &mut [u8]) {
    let te = TimeExceeded::TtlExpired;

    build_time_exceeded(te, bytes)
}
