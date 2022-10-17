# auip

> auip is a async uip stack.

## Layer

### Mac Layer

MAC Layer support these packet type:

- [X] EthernetII
- [X] IEEE802.3
  - [ ] VLAN (802.3q)
  - [ ] QinQ (802.3q)

### Network Layer

Network support these packet and function:

- [X] Ipv4
- [ ] Arp
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

- [ ] Device
- [ ] Interface
- [ ] Socket

### Device

Device is work on mac layer. It only have two function:

- Recv MAC frame, then input to auip. Use poll mode.
- Send MAC frame from auip.

Device is only a trait, you must bind a device to a interface.

``` rust
pub trait Device {
    type Error: Into<Error> + Debug;

    type RecvPacket: AsRef<[u8]>;

    type RecvFuture: Future<Result<mac::Packet<Self::RecvPacket>, Self::Error>>;

    fn recv(&self) -> Self::RecvFuture;

    type SendPacket: AsRef<[u8]> + AsMut<[u8]>;

    fn send(&self, pkt: &Packet<Self::SendPacket>) -> Result<(), Self::Error>;

    fn alloc_packet(&mut self) -> Self::SendPacket;

    fn mac_address(&self) -> mac::Address;
}

```

### Interface

Interface same as linux's interface.

Interface have these features:

- Set IpAddress, CIDR and Gateway.
- Bind with a Device.
- Open Socket on interface.





