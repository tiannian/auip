use crate::{storage::RingCurser, IpFragmentBuffer};

pub struct IpFragmentRingBuffer<IFB> {
    pub ring_curser: RingCurser,
    pub ip_fragment_buffer: IFB,
}

impl<IFB: IpFragmentBuffer> IpFragmentRingBuffer<IFB> {
    pub fn new(ifb: IFB) -> Self {
        let length = ifb.capacity();

        Self {
            ring_curser: RingCurser::new(length),
            ip_fragment_buffer: ifb
        }
    }
}

