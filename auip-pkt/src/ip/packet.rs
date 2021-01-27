use super::arp;
use super::ipv4;
// use super::ipv6;
// use core::convert::TryFrom;
// use crate::mac;
// use crate::error::*;
// use crate::ip;
// use crate::prelude::*;

pub enum Packet<T: AsRef<[u8]>> {
    IPv4(ipv4::Packet<T>),
    ARP(arp::Packet<T>),
    // ipv6(Pac)
}

// impl<T: AsRef<[u8]> + AsMut<[u8]>> TryFrom<mac::ethernet::Packet<T>> for Packet<&mut [u8]> {
//     type Error = Error;

//     fn try_from(pkt: mac::ethernet::Packet<T>) -> Result<Self> {
//         let repr = mac::Repr::try_from(&pkt)?;
//         let pkt_buffer = pkt.payload_mut().unwrap();
//         match repr.protocol {
//             mac::Protocol::IPv4 => {
//                 let ipv4 = ipv4::Packet::new_checked(pkt_buffer).unwrap();
//                 Ok(Packet::IPv4(ipv4))
//             },
//             mac::Protocol::ARP => {
//                 let arp = arp::Packet::new_checked(pkt_buffer).unwrap();
//                 Ok(Packet::ARP(arp))
//             }
//             // mac::Protocol::ARP => Ok(Packet::ARP(arp::Packet::new_checked(pkt.payload_mut())),
//         }
//         // Err(Error::Illegal)
//     }
// }
