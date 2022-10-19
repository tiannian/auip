//! Ethernet packet.

use crate::{prelude::IntoInner, Error, Result};

use super::{consts, Address, Protocol, VlanId};
use byteorder::{ByteOrder, NetworkEndian};
use core::fmt::{self, Display};

/// Ethernet packet.
#[derive(Debug, Clone)]
pub struct Packet<T> {
    buffer: T,
}

mod field {
    use crate::utils::field::Field;

    pub const DESTINATION: Field = 0..6;
    pub const SOURCE: Field = 6..12;
    pub const ETHERTYPE: Field = 12..14;

    pub mod ethernetii {
        use crate::utils::field::Rest;

        pub const PAYLOAD: Rest = 14..;
    }

    pub mod ieee8021q {
        use crate::utils::field::{Field, Rest};

        pub const PRI_CFI_VID: Field = 14..16;
        pub const ETHERTYPE: Field = 16..18;
        pub const PAYLOAD: Rest = 18..;
    }

    pub mod qinq {
        use crate::utils::field::{Field, Rest};

        pub const PRI_CFI_VID: Field = 18..20;
        pub const ETHERTYPE: Field = 20..22;
        pub const PAYLOAD: Rest = 22..;
    }
}

impl<T: AsRef<[u8]>> Display for Packet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Mac Packet:")?;
        f.write_fmt(format_args!(
            "Destination: {}, Source: {}, Protocol: {:?}, Payload Length: {}",
            self.dest_addr(),
            self.src_addr(),
            self.protocol(),
            self.payload_len(),
        ))
    }
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for Packet<T> {
    fn as_ref(&self) -> &[u8] {
        self.buffer.as_ref()
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> AsMut<[u8]> for Packet<T> {
    fn as_mut(&mut self) -> &mut [u8] {
        self.buffer.as_mut()
    }
}

impl<T> IntoInner for Packet<T> {
    type Inner = T;

    fn into_inner(self) -> Self::Inner {
        self.buffer
    }
}

impl<T: AsRef<[u8]>> Packet<T> {
    /// new unchecked packet.
    pub fn new_unchecked(buffer: T) -> Packet<T> {
        Packet { buffer }
    }

    /// new checked packet.
    pub fn new_checked(buffer: T) -> Result<Packet<T>> {
        let packet = Self::new_unchecked(buffer);
        packet.check_len()?;
        Ok(packet)
    }

    fn check_len(&self) -> Result<()> {
        let len = self.buffer.as_ref().len();

        let header_len = self.header_len();

        if len < header_len {
            Err(Error::WrongLengthForEthernetPacket)
        } else {
            Ok(())
        }
    }

    pub fn protocol(&self) -> Protocol {
        let data = self.buffer.as_ref();

        let ty = NetworkEndian::read_u16(&data[field::ETHERTYPE]);

        if ty == consts::IEEE802_1Q {
            let ty = NetworkEndian::read_u16(&data[field::ieee8021q::ETHERTYPE]);

            let vlanid = VlanId::from_bytes_unchecked(&data[field::ieee8021q::PRI_CFI_VID]);

            if ty == consts::IEEE802_1Q {
                let vlanid1 = VlanId::from_bytes_unchecked(&data[field::qinq::PRI_CFI_VID]);
                Protocol::QinQ(vlanid, vlanid1)
            } else {
                Protocol::IEEE8021Q(vlanid)
            }
        } else {
            Protocol::from(ty)
        }
    }

    /*     pub fn upper_protocol(&self) -> Protocol { */
    /*  */
    /* } */

    pub fn header_len(&self) -> usize {
        let protocol = self.protocol();

        match protocol {
            Protocol::IEEE8021Q(_) => field::ieee8021q::PAYLOAD.start,
            Protocol::QinQ(_, _) => field::qinq::PAYLOAD.start,
            _ => field::ethernetii::PAYLOAD.start,
        }
    }

    pub fn payload_len(&self) -> usize {
        let len = self.header_len();

        self.buffer.as_ref().len() - len
    }

    #[inline]
    pub fn dest_addr(&self) -> Address {
        let inner = self.buffer.as_ref();
        (&inner[field::DESTINATION]).into()
    }

    #[inline]
    pub fn src_addr(&self) -> Address {
        let inner = self.buffer.as_ref();
        (&inner[field::SOURCE]).into()
    }

    pub fn payload(&self) -> &[u8] {
        let inner = self.buffer.as_ref();

        let protocol = self.protocol();

        match protocol {
            Protocol::IEEE8021Q(_) => &inner[field::ieee8021q::PAYLOAD],
            Protocol::QinQ(_, _) => &inner[field::qinq::PAYLOAD],
            _ => &inner[field::ethernetii::PAYLOAD],
        }
    }
}

impl<T: AsRef<[u8]> + AsMut<[u8]>> Packet<T> {
    /// set ethernet type.
    pub fn set_protocol(&mut self, protocol: Protocol) {
        let data = self.buffer.as_mut();

        match &protocol {
            Protocol::IEEE8021Q(vlanid) => {
                NetworkEndian::write_u16(&mut data[field::ieee8021q::PRI_CFI_VID], vlanid.0)
            }
            Protocol::QinQ(vlanid, vlanid1) => {
                NetworkEndian::write_u16(&mut data[field::ieee8021q::PRI_CFI_VID], vlanid.0);
                NetworkEndian::write_u16(&mut data[field::qinq::ETHERTYPE], (&protocol).into());
                NetworkEndian::write_u16(&mut data[field::qinq::PRI_CFI_VID], vlanid1.0);
            }
            _ => {}
        }

        NetworkEndian::write_u16(&mut data[field::ETHERTYPE], protocol.into())
    }

    #[inline]
    pub fn set_dest_addr(&mut self, addr: Address) {
        let data = self.buffer.as_mut();
        data[field::DESTINATION].copy_from_slice(addr.as_bytes());
    }

    #[inline]
    pub fn set_src_addr(&mut self, addr: Address) {
        let data = self.buffer.as_mut();
        data[field::SOURCE].copy_from_slice(addr.as_bytes());
    }

    #[inline]
    pub fn payload_mut(&mut self) -> &mut [u8] {
        let protocol = self.protocol();

        let inner = self.buffer.as_mut();

        match protocol {
            Protocol::IEEE8021Q(_) => &mut inner[field::ieee8021q::PAYLOAD],
            Protocol::QinQ(_, _) => &mut inner[field::qinq::PAYLOAD],
            _ => &mut inner[field::ethernetii::PAYLOAD],
        }
    }
}
