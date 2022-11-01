use auip_pkt::{layer2, layer3};

use crate::{Medium, Result};

/// A device for sending and receiving raw packet.
pub trait Device {
    /// Send packet to this device
    fn send(&mut self, buffer: &[u8]) -> Result<()>;

    /// Receive packet from this device.
    fn recv(&mut self) -> Result<Option<&[u8]>>;

    /// Medium type for this device
    fn medium(&self) -> Medium;
}

/// Storage for address
///
/// This storage must be store one mac address and multiple ip address.
pub trait AddrsStorage {
    /// Get mac address from address storage.
    fn mac_addr(&self) -> &layer2::Address;

    /// Checking ip address is exist in address storage
    fn has_ip_addr(&self, ip_addr: &layer3::Address) -> bool;
}

/// Storage for arp table.
pub trait ArpStorage {
    /// Set map of mac address and ip address.
    fn set_map(&mut self, mac: layer2::Address, ip_addr: layer3::ipv4::Address) -> Result<()>;

    /// Get mac address according mac address
    fn mac_addr(&self, ip_addr: &layer3::ipv4::Address) -> Result<Option<layer2::Address>>;
}

/// Buffer to store ip fragment.
pub trait IpFragmentBuffer {
    /// Get ip fragment buffer, buffer length is 64k
    fn get_buffer(&mut self, ident: u16) -> Option<&mut [u8]>;
}
