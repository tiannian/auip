# auip

> auip is a async uip stack.

## Layer

### Mac Layer

MAC Layer support these packet type:

- [X] EthernetII
- [X] IEEE802.3
  - [X] VLAN (802.3q)
  - [X] QinQ (802.3q)

### Network Layer

Network support these packet and function:

- [X] Ipv4
- [X] Arp
- [ ] ICMP
- [ ] Ipv6
- [ ] ICMPv6 (NDP)

### Transport Layer

- [ ] UDP

### Application Layer

- [ ] DHCP
- [ ] DHCPv6
- [ ] DNS

## Architecture

- [X] Device
- [X] Interface
- [X] Storage
- [ ] Hook
- [ ] Socket

### Device

Device is work on mac layer. It only have two function:

- Recv MAC frame, then input to auip. Use poll mode.
- Send MAC frame from auip.

Device is only a trait, you must bind a device to a interface.

``` rust
pub trait Device {
    fn send(&mut self, buffer: &[u8]) -> Result<()>;

    fn recv(&mut self) -> Result<Option<&[u8]>>;

    fn medium(&self) -> Medium;
}
```

Currently support these device:

- [X] Tap (Linux, MacOS)
- [X] Tun (Linux, MacOS)
- [ ] RawSocket (Windows)
- [ ] Bridge
- [ ] Loopback
- [ ] Sub-port (For macvtap, vlan)

### Interface

Interface same as linux's interface.

Interface have these functions

- Set CIDR and MAC Address.
- Bind with a Device
  - Receive packet from device, then send to socket.
  - Send ip packet to device.
- Drop or accept packet based on
  - Vlan ID
  - Mac Address
  - IP Address
- Hook baseed on process pcaket.

### Storage

Beacuse auip support both nostd and alloc, all storage declared as trait.

- AddrsStorage: Storage addresses for interface, include 1 mac address and multiple cidr.
- Layer3PacketStorage: As a buffer to store layer3 packet need send to device.

### Hook

Based on packet process, interface can register a hook. Hook will include some function,
these function will called when packet is procedded.

Throw hook, we can build macvlan, vlan device, bridge, switch or some other special network interface. 
Also, we can use hook to build cross interface packet route, 6lowpan gateway, ipv6 on ble gateway and so on.

Beacuse the fundamental of auip is direct stack model, we need use hook to cross packet.

