use crate::error::*;

/// Convert into.
pub trait IntoInner {
    type Inner;
    fn into_inner(self) -> Self::Inner;
}

/// Get destination address.
pub trait DestAddr {
    type Address;

    fn dest_addr(&self) -> Result<Self::Address>;
}

/// Get source asddress.
pub trait SrcAddr {
    type Address;

    fn src_addr(&self) -> Result<Self::Address>;
}

/// Get payload
pub trait Payload {
    type Payload: ?Sized;

    fn payload(&self) -> Result<&Self::Payload>;
}

/// Set destination address.
pub trait DestAddrMut {
    type Address;

    fn set_dest_addr(&mut self, addr: &Self::Address) -> Result<()>;
}

/// Set source address.
pub trait SrcAddrMut {
    type Address;

    fn set_src_addr(&mut self, addr: &Self::Address) -> Result<()>;
}

/// Get mutable payload bytes.
pub trait PayloadMut {
    type Payload: ?Sized;

    fn payload_mut(&mut self) -> Result<&mut Self::Payload>;
}