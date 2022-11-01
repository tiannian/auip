use alloc::vec::Vec;

use crate::{consts::MAX_IP_FRAGMENT_PACKET_LENGTH, IpFragmentBuffer};

pub struct IpFragment {
    pub max_length: usize,
    pub buffers: Vec<u8>,
    // pub mapping:
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
    fn get_buffer(&mut self, idx: usize) -> Option<&mut [u8]> {
        if idx < self.max_length {
            let end = (idx + 1) * MAX_IP_FRAGMENT_PACKET_LENGTH;
            let begin = idx * MAX_IP_FRAGMENT_PACKET_LENGTH;

            if end > self.buffers.len() {
                self.buffers.resize(end, 0);
            }

            Some(&mut self.buffers[begin..end])
        } else {
            None
        }
    }
}
