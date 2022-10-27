use alloc::vec::Vec;

use crate::{consts::MAX_IP_FRAGMENT_PACKET_LENGTH, utils::FixedBytes, IpFragmentBuffer};

pub struct IpFragment {
    pub max_length: usize,
    pub buffers: Vec<FixedBytes<{ MAX_IP_FRAGMENT_PACKET_LENGTH }>>,
}

impl IpFragment {
    pub fn new(max_length: usize) -> Self {
        Self {
            max_length,
            // TODO: Opt
            buffers: Vec::new(),
        }
    }
}

impl IpFragmentBuffer for IpFragment {
    fn capacity(&self) -> usize {
        self.max_length
    }

    fn get_buffer(&mut self, idx: usize) -> Option<&mut [u8]> {
        if idx < self.max_length {
            None
        } else {
            None
        }
    }
}
