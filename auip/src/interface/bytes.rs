use core::ops::Deref;

use auip_pkt::{layer2, layer3, layer4};

use crate::consts;

macro_rules! define_bytes {
    ($name:ident, $length:expr) => {
        pub struct $name(pub [u8; $length]);

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl AsMut<[u8]> for $name {
            fn as_mut(&mut self) -> &mut [u8] {
                &mut self.0
            }
        }

        impl Deref for $name {
            type Target = [u8];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self([0u8; $length])
            }
        }
    };
}

const MIN_ARP_PACKET_LEN: usize =
    layer2::ethernet::field::MAX_HEADER_LEN + layer3::arp::field::PACKET_LEN;
define_bytes!(ArpBytes, MIN_ARP_PACKET_LEN);

define_bytes!(NoFragIpBytes, consts::NO_FRAG_PACKET_LENGTH);

const ICMPV4_PACKET_LENGTH: usize = layer3::ipv4::field::HEADER_LEN_WITHOUT_OPTION as usize + layer4::icmpv4::field::HEADER_END;
define_bytes!(Icmpv4Bytes, ICMPV4_PACKET_LENGTH);


