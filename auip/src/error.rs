use core::fmt::Debug;

#[derive(Debug)]
pub enum Error {
    NoSpaceForAddrsStorage,

    PacketError(auip_pkt::Error),
}

impl From<auip_pkt::Error> for Error {
    fn from(p: auip_pkt::Error) -> Self {
        Self::PacketError(p)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
