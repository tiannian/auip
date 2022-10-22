use core::fmt::Debug;

/// Auip error
#[derive(Debug)]
pub enum Error {
    NoSpaceForAddrsStorage,

    NoSpaceForArpStorage,

    UnexpectedType,

    IpAddrNotFound,

    NoVlanIdSet,

    PacketError(auip_pkt::Error),
}

impl From<auip_pkt::Error> for Error {
    fn from(p: auip_pkt::Error) -> Self {
        Self::PacketError(p)
    }
}

/// Result for auip
pub type Result<T> = core::result::Result<T, Error>;
