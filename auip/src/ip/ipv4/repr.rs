use super::Address;

pub struct Repr {
    pub destination: Address,
    pub source: Address,
    pub ttl: u8,
}
