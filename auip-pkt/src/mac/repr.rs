use super::{Address, ethernet, Protocol};
use crate::prelude::*;
use crate::error::*;
use core::convert::TryFrom;

pub struct Repr {
    pub destination: Address,
    pub source: Address,
    pub protocol: Protocol,
}

impl<T: AsRef<[u8]>> TryFrom<&ethernet::Packet<T>> for Repr {
    type Error = Error;

    fn try_from(pkt: &ethernet::Packet<T>) -> Result<Repr> {
        Ok(Repr {
            destination: pkt.dest_addr()?,
            source: pkt.src_addr()?,
            protocol: pkt.ethernet_type(),
        })
    }
}

// impl<T: AsRef<[u8]>> From<&ethernet::Packet<T>> for Repr {
//     fn from(pkt: &ethernet::Packet<T>) -> Repr {
//         Repr {
//             destination: pkt.dest_addr().unwrap(),
//             source: pkt.src_addr().unwrap(),
//             protocol: pkt.ethernet_type(),
//         }
//     }
// }
