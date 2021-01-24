#![no_std]

// pub mod phy;
pub mod ip;
pub mod mac;
// pub mod transport;

/// Mac EthernetII
/// Ip Arp ICMP IGMP
/// Tp UDP TCP

pub trait FrameUp {
    type Upper;
    fn up(&self) -> Self::Upper;
}

pub trait FrameDown<F> {
    fn down(&self) -> F;
}

pub trait ToRepr {
    type Repr;

    fn to_repr(&self) -> Self::Repr;
}

pub trait FromRepr {
    type Packet;

    fn from_repr(&self, packet: &mut Self::Packet);
}

pub type Field = core::ops::Range<usize>;
pub type Rest = core::ops::RangeFrom<usize>;

#[derive(Debug)]
pub enum Error {
    Truncated,
}

pub type Result<R> = core::result::Result<R, Error>;
