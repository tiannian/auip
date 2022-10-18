/// Error's for packet.
#[derive(Clone, Debug)]
pub enum Error {
    Truncated,
    Illegal,
    Malformed,
    Unrecognized,
    WrongLengthForEthernetAddress,
    WrongLengthForIpv4Address,
}

/// Result for packet.
pub type Result<R> = core::result::Result<R, Error>;
