//! Utils

#[derive(Debug, Clone, Copy)]
pub struct FixedBytes<const LEN: usize>(pub [u8; LEN]);

impl<const LEN: usize> Default for FixedBytes<LEN> {
    fn default() -> Self {
        Self([0u8; LEN])
    }
}

impl<const LEN: usize> AsRef<[u8]> for FixedBytes<LEN> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const LEN: usize> AsMut<[u8]> for FixedBytes<LEN> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}
