/// Error's for packet.
#[derive(Clone, Debug)]
pub enum Error {
    WrongLengthForEthernetAddress,
    WrongLengthForIpv4Address,
    WrongLengthForArpPacket,
    WrongLengthForIpv4Packet,
    WrongLengthForEthernetPacket,
    UnknownIpVersionNumber,
    IllegalNetmask,
    ParseMacAddressFailed,
    ParseIpv4CidrFailed,
    ParseIpv4AddressFailed,
    ParseIntError(core::num::ParseIntError),
}

impl From<core::num::ParseIntError> for Error {
    fn from(e: core::num::ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

/// Result for packet.
pub type Result<R> = core::result::Result<R, Error>;
