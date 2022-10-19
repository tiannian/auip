/// Error's for packet.
#[derive(Clone, Debug)]
pub enum Error {
    Illegal,
    Malformed,
    Unrecognized,
    WrongLengthForEthernetAddress,
    WrongLengthForIpv4Address,
    WrongLengthForArpPacket,
    WrongLengthForIpv4Packet,
    WrongLengthForEthernetPacket,
}

/// Result for packet.
pub type Result<R> = core::result::Result<R, Error>;
