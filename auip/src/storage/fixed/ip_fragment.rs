use crate::{consts::MAX_IP_FRAGMENT_PACKET_LENGTH, utils::FixedBytes, IpFragmentBuffer};

pub struct IpFragment<const N: usize> {
    pub buffers: [FixedBytes<{ MAX_IP_FRAGMENT_PACKET_LENGTH }>; N],
}

impl<const N: usize> Default for IpFragment<N> {
    fn default() -> Self {
        Self {
            buffers: [Default::default(); N],
        }
    }
}

impl<const N: usize> IpFragmentBuffer for IpFragment<N> {
    fn capacity(&self) -> usize {
        N
    }

    fn get_buffer(&mut self, idx: usize) -> Option<&mut [u8]> {
        self.buffers.get_mut(idx).map(|v| v.as_mut())
    }
}
