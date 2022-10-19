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
}

/// Result for packet.
pub type Result<R> = core::result::Result<R, Error>;
