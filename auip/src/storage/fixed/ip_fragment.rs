use crate::{consts::MAX_IP_FRAGMENT_PACKET_LENGTH, utils::FixedBytes, IpFragmentBuffer};

pub struct IpFragment<const N: usize> {
    /// Buffers for ip fragment.
    ///
    /// We can use [u8; MAX_IP_FRAGMENT_PACKET_LENGTH * N]. But avoid use unstable features.
    pub buffers: [FixedBytes<{ MAX_IP_FRAGMENT_PACKET_LENGTH }>; N],

    /// Mapping of idnet.
    pub mapping: [u16; N],
}

impl<const N: usize> Default for IpFragment<N> {
    fn default() -> Self {
        Self {
            buffers: [Default::default(); N],
            mapping: [0u16; N],
        }
    }
}

impl<const N: usize> IpFragmentBuffer for IpFragment<N> {
    fn get_buffer(&mut self, ident: u16) -> Option<&mut [u8]> {
        if let Ok(idx) = self.mapping.binary_search(&ident) {
            self.buffers.get_mut(idx).map(|v| v.as_mut())
        } else {
            None
        }
    }
}
