use alloc::{collections::BTreeMap, vec::Vec};

use crate::{consts::MAX_IP_FRAGMENT_PACKET_LENGTH, IpFragmentBuffer};

pub struct IpFragment {
    max_length: usize,
    buffers: Vec<u8>,
    mapping: BTreeMap<u16, usize>,
    curser: usize,
}

impl IpFragment {
    pub fn new(max_length: usize) -> Self {
        Self {
            max_length,
            buffers: Vec::new(),
            mapping: BTreeMap::new(),
            curser: 0,
        }
    }
}

impl IpFragmentBuffer for IpFragment {
    fn get_buffer(&mut self, ident: u16) -> &mut [u8] {
        let curser = if self.max_length == 1 {
            0
        } else if let Some(curser) = self.mapping.get(&ident) {
            *curser
        } else {
            self.curser += 1;
            if self.curser == self.max_length {
                self.curser = 0;
            }
            log::info!("Current curser is {}, map to {}", self.curser, ident);
            self.mapping.insert(ident, self.curser);
            self.curser
        };

        let begin = curser * MAX_IP_FRAGMENT_PACKET_LENGTH;
        let end = (curser + 1) * MAX_IP_FRAGMENT_PACKET_LENGTH;

        self.buffers.resize(end, 0);

        &mut self.buffers[begin..end]
    }
}
