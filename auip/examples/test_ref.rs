#[derive(Default)]
struct Device {
    inner: [u8; 15],
}

impl Device {
    pub fn get_mut_ref(&mut self) -> &mut [u8] {
        &mut self.inner
    }

    pub fn get_ref(&self) -> &[u8] {
        &self.inner
    }

    pub fn receive(&self) -> Layer3Packet<&[u8]> {
        let buffer = self.get_ref();
        let layer2_pkt = Layer2Packet::from_bytes(buffer);
        let buffer2 = layer2_pkt.get_ref();
        let layer3_pkt = Layer3Packet::from_bytes(buffer2);
        layer3_pkt
    }
}

struct Layer2Packet<T: AsRef<[u8]>> {
    buffer: T,
}

impl<'a, T: AsRef<[u8]> + ?Sized> Layer2Packet<&'a T> {
    pub fn get_ref(&self) -> &'a [u8] {
        self.buffer.as_ref()
    }
}

impl<T: AsRef<[u8]>> Layer2Packet<T> {
    pub fn from_bytes(buffer: T) -> Self {
        Self { buffer }
    }
}

struct Layer3Packet<T: AsRef<[u8]>> {
    buffer: T,
}

impl<'a, T: AsRef<[u8]> + ?Sized> Layer3Packet<&'a T> {
    pub fn get_ref(&self) -> &'a [u8] {
        self.buffer.as_ref()
    }
}
impl<T: AsRef<[u8]>> Layer3Packet<T> {
    pub fn from_bytes(buffer: T) -> Self {
        Self { buffer }
    }
}

fn main() {}
