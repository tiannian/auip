use crate::{consts::MAX_IP_FRAGMENT_PACKET_LENGTH, utils::FixedBytes, IpFragmentBuffer};

pub struct IpFragment<const N: usize> {
    /// Buffers for ip fragment.
    ///
    /// We can use [u8; MAX_IP_FRAGMENT_PACKET_LENGTH * N]. But avoid use unstable features.
    buffers: [FixedBytes<{ MAX_IP_FRAGMENT_PACKET_LENGTH }>; N],

    /// Mapping of idnet.
    mapping: [u16; N],

    /// Curser
    curser: usize,
}

impl<const N: usize> Default for IpFragment<N> {
    fn default() -> Self {
        assert!(N > 0, "Ip fragment must at least 1");

        Self {
            buffers: [Default::default(); N],
            mapping: [0u16; N],
            curser: 0,
        }
    }
}

impl<const N: usize> IpFragmentBuffer for IpFragment<N> {
    fn get_buffer(&mut self, ident: u16) -> &mut [u8] {
        let curser = if N == 1 {
            0
        } else if let Ok(curser) = self.mapping.binary_search(&ident) {
            curser
        } else {
            self.curser += 1;
            if self.curser == N {
                self.curser = 0;
            }
            log::debug!("Current curser is {}, map to {}", self.curser, ident);
            self.mapping[self.curser] = ident;
            self.curser
        };
        self.buffers[curser].as_mut()
    }
}
