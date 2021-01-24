#![no_std]

pub mod ip;
pub mod mac;
pub mod utils;

/// Field
pub type Field = core::ops::Range<usize>;
/// Rest
pub type Rest = core::ops::RangeFrom<usize>;

/// Convert into.
pub trait IntoInner {
    type Inner;
    fn into_inner(self) -> Self::Inner;
}

/// Get destination address.
pub trait DestAddr {
    type Address;

    fn dest_addr(&self) -> Self::Address;
}

/// Get source asddress.
pub trait SrcAddr {
    type Address;

    fn src_addr(&self) -> Self::Address;
}

/// Get payload
pub trait Payload {
    type Payload: ?Sized;

    fn payload(&self) -> &Self::Payload;
}

/// Set destination address.
pub trait DestAddrMut {
    type Address;

    fn set_dest_addr(&mut self, addr: &Self::Address);
}

/// Set source address.
pub trait SrcAddrMut {
    type Address;

    fn set_src_addr(&mut self, addr: &Self::Address);
}

/// Get mutable payload bytes.
pub trait PayloadMut {
    type Payload: ?Sized;

    fn payload_mut(&mut self) -> &mut Self::Payload;
}

/// Error's for packet.
pub enum Error {
    Truncated,
    Illegal,
    Malformed,
}

/// Result for packet.
pub type Result<R> = core::result::Result<R, Error>;
