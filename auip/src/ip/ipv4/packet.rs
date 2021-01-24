#[derive(Debug, Clone)]
pub struct Packet<P: AsRef<[u8]>> {
    data: P,
}

impl<P: AsRef<[u8]>> AsRef<[u8]> for Packet<P> {
    fn as_ref(&self) -> &[u8] {
        self.data.as_ref()
    }
}

impl<P: AsRef<[u8]>> Packet<P> {
    pub fn new(p: P) -> Self {
        Self { data: p }
    }
}
