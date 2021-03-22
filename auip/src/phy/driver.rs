//! phy layer.

use auip_pkt::mac;
use core::pin::Pin;
use core::task::{Context, Poll};
use crate::Result;
// use auip_pkt::ip;

/// Device work layer.
pub enum DeviceLayer {
    Layer2,
    Layer3,
}

/// Device capabilities.
pub struct DeviceCapabilities {
    pub max_transmission_unit: usize,
    pub max_burst_size: Option<usize>,
    pub layer: DeviceLayer,
}

/// Device trait.
///
/// Device receive packet from device, then send mac repr to stack.
/// Device receive mac repr fomr stack, then send packet to device.
pub trait Driver {
    fn capabilities(&self) -> DeviceCapabilities;

    fn poll_recv(self: Pin<&mut Self>, cx: Context<'_>) -> Poll<Result<mac::Packet<&'_ mut [u8]>>>;

    fn poll_send(self: Pin<&mut Self>, cx: Context<'_>, pkt: mac::Packet<&'_ mut [u8]>)
        -> Poll<Result<()>>;
}
