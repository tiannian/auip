use crate::{Medium, Result};

pub trait Device {
    fn send(&mut self, buffer: &[u8]) -> Result<()>;

    fn recv(&mut self) -> Result<&[u8]>;

    fn medium(&self) -> Medium;
}
