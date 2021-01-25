//! phy layer.

use core::future::Future;
use auip_pkt::mac::Repr;

/// Device work layer.
pub enum DeviceLayer {
    Mac,
    Ip,
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
pub trait Device {
    fn capabilities(&self) -> DeviceCapabilities;

    type ReturnReceiveFuture<'__async_trait>: Future<Output = (Repr, &'__async_trait mut [u8])>;
    fn receive<'__async_trait>(
        &'__async_trait mut self,
    ) -> Self::ReturnReceiveFuture<'__async_trait>;
}
