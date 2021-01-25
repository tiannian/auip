/// Error's for packet.
pub enum Error {
    Truncated,
    Illegal,
    Malformed,
}

/// Result for packet.
pub type Result<R> = core::result::Result<R, Error>;