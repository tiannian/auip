#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Hardware {
    Ethernet,
    Unknown(u16),
}

impl From<u16> for Hardware {
    fn from(v: u16) -> Self {
        match v {
            1 => Hardware::Ethernet,
            _ => Hardware::Unknown(v),
        }
    }
}

impl From<Hardware> for u16 {
    fn from(v: Hardware) -> u16 {
        match v {
            Hardware::Ethernet => 1,
            Hardware::Unknown(a) => a,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    Request,
    Reply,
    Unknown(u16),
}

impl From<u16> for Operation {
    fn from(v: u16) -> Self {
        match v {
            1 => Operation::Request,
            2 => Operation::Reply,
            _ => Operation::Unknown(v),
        }
    }
}

impl From<Operation> for u16 {
    fn from(v: Operation) -> u16 {
        match v {
            Operation::Request => 1,
            Operation::Reply => 2,
            Operation::Unknown(a) => a,
        }
    }
}
