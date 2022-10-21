use core::ops::Deref;

use auip_pkt::{layer2, layer3};

const MIN_ARP_PACKET_LEN: usize =
    layer2::ethernet::field::MAX_HEADER_LEN + layer3::arp::field::PACKET_LEN;

pub struct ArpBytes(pub [u8; MIN_ARP_PACKET_LEN]);

impl AsRef<[u8]> for ArpBytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for ArpBytes {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Deref for ArpBytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for ArpBytes {
    fn default() -> Self {
        Self([0u8; MIN_ARP_PACKET_LEN])
    }
}

pub enum Action {
    NoAction,
    SendArp(ArpBytes),
}
