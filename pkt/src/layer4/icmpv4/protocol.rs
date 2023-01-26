pub mod consts {
    pub mod message {
        pub const ECHO_REPLY: u8 = 0;
        pub const DST_UNREACHABLE: u8 = 3;
        pub const REDIRECT: u8 = 5;
        pub const ECHO_REQUEST: u8 = 8;
        pub const ROUTER_ADVERT: u8 = 9;
        pub const ROUTER_SOLICIT: u8 = 10;
        pub const TIME_EXCEEDED: u8 = 11;
        pub const PARAM_PROBLEM: u8 = 12;
        pub const TIMESTAMP: u8 = 13;
        pub const TIMESTAMP_REPLY: u8 = 14;
    }

    pub mod dst_unreachable {
        pub const NET_UNREACHABLE: u8 = 0;
        pub const HOST_UNREACHABLE: u8 = 1;
        pub const PROTO_UNREACHABLE: u8 = 2;
        pub const PORT_UNREACHABLE: u8 = 3;
        pub const FRAG_REQUIRED: u8 = 4;
        pub const SRC_ROUTE_FAILED: u8 = 5;
        pub const DST_NET_UNKNOWN: u8 = 6;
        pub const DST_HOST_UNKNOWN: u8 = 7;
        pub const SRC_HOST_ISOLATED: u8 = 8;
        pub const NET_PROHIBITED: u8 = 9;
        pub const HOST_PROHIBITED: u8 = 10;
        pub const NET_UNREACH_TOS: u8 = 11;
        pub const HOST_UNREACH_TOS: u8 = 12;
        pub const COMM_PROHIBITED: u8 = 13;
        pub const HOST_PRECED_VIOL: u8 = 14;
        pub const PRECED_CUTOFF: u8 = 15;
    }

    pub mod redirect {
        pub const NET: u8 = 0;
        pub const HOST: u8 = 1;
        pub const NET_TO_S: u8 = 2;
        pub const HOST_TO_S: u8 = 3;
    }

    pub mod time_exceeded {
        pub const TTL_EXPIRED: u8 = 0;
        pub const FRAG_EXPIRED: u8 = 1;
    }

    pub mod param_problem {
        pub const AT_POINTER: u8 = 0;
        pub const MISSING_OPTION: u8 = 1;
        pub const BAD_LENGTH: u8 = 2;
    }
}

/// Echo
#[derive(Debug, Clone, Default)]
pub struct Echo {
    pub ident: u16,
    pub seq_no: u16,
}

/// Internet protocol control message type.
#[derive(Debug, Clone)]
pub enum Message {
    /// Echo request
    EchoRequest(Echo),
    /// Echo reply
    EchoReply(Echo),
    /// Destination unreachable
    DstUnreachable(DstUnreachable),
    /// Message redirect
    Redirect(Redirect),
    /// Router advertisement
    RouterAdvert,
    /// Router solicitation
    RouterSolicit,
    /// Time exceeded
    TimeExceeded(TimeExceeded),
    /// Parameter problem
    ParamProblem(ParamProblem),
    /// Timestamp
    Timestamp,
    /// Timestamp reply
    TimestampReply,
    /// Unknown
    Unknown(u8, u8),
}

impl Message {
    pub fn from_type_code(ty: u8, code: u8) -> Self {
        match ty {
            consts::message::ECHO_REPLY => Self::EchoReply(Echo::default()),
            consts::message::DST_UNREACHABLE => {
                Self::DstUnreachable(DstUnreachable::from_code(code))
            }
            consts::message::REDIRECT => Self::Redirect(Redirect::from_code(code)),
            consts::message::ECHO_REQUEST => Self::EchoRequest(Echo::default()),
            consts::message::ROUTER_ADVERT => Self::RouterAdvert,
            consts::message::ROUTER_SOLICIT => Self::RouterSolicit,
            consts::message::TIME_EXCEEDED => Self::TimeExceeded(TimeExceeded::from_code(code)),
            consts::message::PARAM_PROBLEM => Self::ParamProblem(ParamProblem::from_code(code)),
            consts::message::TIMESTAMP => Self::Timestamp,
            consts::message::TIMESTAMP_REPLY => Self::TimestampReply,
            _ => Self::Unknown(ty, code),
        }
    }

    pub fn to_type_code(&self) -> (u8, u8) {
        match self {
            Self::EchoRequest(_) => (consts::message::ECHO_REQUEST, 0),
            Self::EchoReply(_) => (consts::message::ECHO_REPLY, 0),
            Self::Timestamp => (consts::message::TIMESTAMP, 0),
            Self::TimestampReply => (consts::message::TIMESTAMP_REPLY, 0),
            Self::RouterSolicit => (consts::message::ROUTER_SOLICIT, 0),
            Self::RouterAdvert => (consts::message::ROUTER_ADVERT, 0),
            Self::DstUnreachable(v) => (consts::message::DST_UNREACHABLE, v.to_code()),
            Self::Redirect(v) => (consts::message::REDIRECT, v.to_code()),
            Self::TimeExceeded(v) => (consts::message::TIME_EXCEEDED, v.to_code()),
            Self::ParamProblem(v) => (consts::message::PARAM_PROBLEM, v.to_code()),
            Self::Unknown(v1, v2) => (*v1, *v2),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DstUnreachable {
    /// Destination network unreachable
    NetUnreachable,
    /// Destination host unreachable
    HostUnreachable,
    /// Destination protocol unreachable
    ProtoUnreachable,
    /// Destination port unreachable
    PortUnreachable,
    /// Fragmentation required, and DF flag set
    FragRequired,
    /// Source route failed
    SrcRouteFailed,
    /// Destination network unknown
    DstNetUnknown,
    /// Destination host unknown
    DstHostUnknown,
    /// Source host isolated
    SrcHostIsolated,
    /// Network administratively prohibited
    NetProhibited,
    /// Host administratively prohibited
    HostProhibited,
    /// Network unreachable for ToS
    NetUnreachToS,
    /// Host unreachable for ToS
    HostUnreachToS,
    /// Communication administratively prohibited
    CommProhibited,
    /// Host precedence violation
    HostPrecedViol,
    /// Precedence cutoff in effect
    PrecedCutoff,
    /// Unknown
    Unknown(u8),
}

impl DstUnreachable {
    pub fn from_code(code: u8) -> Self {
        match code {
            consts::dst_unreachable::NET_UNREACHABLE => Self::NetUnreachable,
            consts::dst_unreachable::HOST_UNREACHABLE => Self::HostUnreachable,
            consts::dst_unreachable::PROTO_UNREACHABLE => Self::ProtoUnreachable,
            consts::dst_unreachable::PORT_UNREACHABLE => Self::PortUnreachable,
            consts::dst_unreachable::FRAG_REQUIRED => Self::FragRequired,
            consts::dst_unreachable::SRC_ROUTE_FAILED => Self::SrcRouteFailed,
            consts::dst_unreachable::DST_NET_UNKNOWN => Self::DstNetUnknown,
            consts::dst_unreachable::DST_HOST_UNKNOWN => Self::DstHostUnknown,
            consts::dst_unreachable::SRC_HOST_ISOLATED => Self::SrcHostIsolated,
            consts::dst_unreachable::NET_PROHIBITED => Self::NetProhibited,
            consts::dst_unreachable::HOST_PROHIBITED => Self::HostProhibited,
            consts::dst_unreachable::NET_UNREACH_TOS => Self::NetUnreachToS,
            consts::dst_unreachable::HOST_UNREACH_TOS => Self::HostUnreachToS,
            consts::dst_unreachable::COMM_PROHIBITED => Self::CommProhibited,
            consts::dst_unreachable::HOST_PRECED_VIOL => Self::HostPrecedViol,
            consts::dst_unreachable::PRECED_CUTOFF => Self::PrecedCutoff,
            _ => Self::Unknown(code),
        }
    }

    pub fn to_code(&self) -> u8 {
        match self {
            Self::NetUnreachable => consts::dst_unreachable::NET_UNREACHABLE,
            Self::HostUnreachable => consts::dst_unreachable::HOST_UNREACHABLE,
            Self::ProtoUnreachable => consts::dst_unreachable::PROTO_UNREACHABLE,
            Self::PortUnreachable => consts::dst_unreachable::PORT_UNREACHABLE,
            Self::FragRequired => consts::dst_unreachable::FRAG_REQUIRED,
            Self::SrcRouteFailed => consts::dst_unreachable::SRC_ROUTE_FAILED,
            Self::DstNetUnknown => consts::dst_unreachable::DST_NET_UNKNOWN,
            Self::DstHostUnknown => consts::dst_unreachable::DST_HOST_UNKNOWN,
            Self::SrcHostIsolated => consts::dst_unreachable::SRC_HOST_ISOLATED,
            Self::NetProhibited => consts::dst_unreachable::NET_PROHIBITED,
            Self::HostProhibited => consts::dst_unreachable::HOST_PROHIBITED,
            Self::NetUnreachToS => consts::dst_unreachable::NET_UNREACH_TOS,
            Self::HostUnreachToS => consts::dst_unreachable::HOST_UNREACH_TOS,
            Self::CommProhibited => consts::dst_unreachable::COMM_PROHIBITED,
            Self::HostPrecedViol => consts::dst_unreachable::HOST_PRECED_VIOL,
            Self::PrecedCutoff => consts::dst_unreachable::PRECED_CUTOFF,
            Self::Unknown(n) => *n,
        }
    }
}

/// Internet protocol control message subtype for type "Redirect Message".
#[derive(Debug, Clone)]
pub enum Redirect {
    /// Redirect Datagram for the Network
    Net,
    /// Redirect Datagram for the Host
    Host,
    /// Redirect Datagram for the ToS & network
    NetToS,
    /// Redirect Datagram for the ToS & host
    HostToS,
    /// Unknown
    Unknown(u8),
}

impl Redirect {
    pub fn from_code(code: u8) -> Self {
        match code {
            consts::redirect::HOST => Self::Host,
            consts::redirect::NET => Self::Net,
            consts::redirect::HOST_TO_S => Self::HostToS,
            consts::redirect::NET_TO_S => Self::NetToS,
            _ => Self::Unknown(code),
        }
    }

    pub fn to_code(&self) -> u8 {
        match self {
            Self::Host => consts::redirect::HOST,
            Self::Net => consts::redirect::NET,
            Self::NetToS => consts::redirect::NET_TO_S,
            Self::HostToS => consts::redirect::HOST_TO_S,
            Self::Unknown(v) => *v,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TimeExceeded {
    /// TTL expired in transit
    TtlExpired,
    /// Fragment reassembly time exceeded
    FragExpired,

    /// Unknown
    Unknown(u8),
}

impl TimeExceeded {
    pub fn from_code(code: u8) -> Self {
        match code {
            consts::time_exceeded::TTL_EXPIRED => Self::TtlExpired,
            consts::time_exceeded::FRAG_EXPIRED => Self::FragExpired,
            _ => Self::Unknown(code),
        }
    }

    pub fn to_code(&self) -> u8 {
        match self {
            Self::TtlExpired => consts::time_exceeded::TTL_EXPIRED,
            Self::FragExpired => consts::time_exceeded::FRAG_EXPIRED,
            Self::Unknown(u) => *u,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ParamProblem {
    /// Pointer indicates the error
    AtPointer,
    /// Missing a required option
    MissingOption,
    /// Bad length
    BadLength,

    /// Unknown
    Unknown(u8),
}

impl ParamProblem {
    pub fn from_code(code: u8) -> Self {
        match code {
            consts::param_problem::AT_POINTER => Self::AtPointer,
            consts::param_problem::BAD_LENGTH => Self::BadLength,
            consts::param_problem::MISSING_OPTION => Self::MissingOption,
            _ => Self::Unknown(code),
        }
    }

    pub fn to_code(&self) -> u8 {
        match self {
            Self::AtPointer => consts::param_problem::AT_POINTER,
            Self::MissingOption => consts::param_problem::MISSING_OPTION,
            Self::BadLength => consts::param_problem::BAD_LENGTH,
            Self::Unknown(u) => *u,
        }
    }
}
