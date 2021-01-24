use super::ipv4;

#[derive(Debug, Clone)]
pub enum Packet<P: AsRef<[u8]>> {
    IPv4(ipv4::Packet<P>),
}
