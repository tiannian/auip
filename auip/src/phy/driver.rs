//! phy layer.

use auip_pkt::mac;
use core::future::Future;
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

    type ReturnReceiveFuture<'__async_trait>: Future<Output = Option<mac::Packet<&'__async_trait [u8]>>>;

    fn receive(&mut self) -> Self::ReturnReceiveFuture<'_>;
}
