use auip_pkt::layer3;

use crate::{FixedBytes, Layer3PacketStorage};

pub struct FixedLayer3PacketStorage<const MTU: usize, const LEN: usize> {
    pub packets: [layer3::Packet<FixedBytes<MTU>>; LEN],
}

impl<const MTU: usize, const LEN: usize> Layer3PacketStorage
    for FixedLayer3PacketStorage<MTU, LEN>
{
    type Layer3PacketBytes = FixedBytes<MTU>;

    fn get(&self, idx: usize) -> Option<&layer3::Packet<Self::Layer3PacketBytes>> {
        if self.packets.len() < idx {
            Some(&self.packets[idx])
        } else {
            None
        }
    }

    fn get_mut(&mut self, idx: usize) -> Option<&mut layer3::Packet<Self::Layer3PacketBytes>> {
        if self.packets.len() < idx {
            Some(&mut self.packets[idx])
        } else {
            None
        }
    }

    fn length(&self) -> usize {
        LEN
    }
}
