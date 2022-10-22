use auip_pkt::layer2::VlanId;

/// Config for interface
#[derive(Debug, Default)]
pub struct InterfaceConfig {
    pub vlan: VlanConfig,
}

/// Config vlan for interface
#[derive(Debug, Default)]
pub struct VlanConfig {
    pub vlanid0: Option<VlanId>,
    pub vlanid1: Option<VlanId>,

    pub tag_vlan0: bool,
    pub tag_vlan1: bool,
}
