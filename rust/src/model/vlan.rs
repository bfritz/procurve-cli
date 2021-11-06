use std::fmt;

#[derive(Debug)]
pub struct Vlan {
    pub id: u16,
    pub name: String,
}

impl Vlan {
    pub fn new(id: u16, name: &str) -> Vlan {
        assert!(id < 4096);
        Vlan {
            id,
            name: name.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct VlanPortParticipation {
    pub vlan_id: u16,
    pub ports: Vec<VlanPortConfig>,
}

impl VlanPortParticipation {
    pub fn new(vlan_id: u16, ports: Vec<VlanPortConfig>) -> VlanPortParticipation {
        assert!(vlan_id < 4096);
        VlanPortParticipation { vlan_id, ports }
    }
}

#[derive(Clone, Debug)]
pub struct VlanPortConfig {
    pub mode: VlanMode,
}

impl VlanPortConfig {
    pub fn new(mode: VlanMode) -> VlanPortConfig {
        VlanPortConfig { mode }
    }
}

#[derive(Clone, Debug)]
pub enum VlanMode {
    Excluded,
    Tagged,
    Untagged,
}

impl fmt::Display for VlanMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VlanMode::Excluded => write!(f, "E"),
            VlanMode::Tagged => write!(f, "T"),
            VlanMode::Untagged => write!(f, "U"),
        }
    }
}
