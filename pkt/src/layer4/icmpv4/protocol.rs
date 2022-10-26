pub mod consts {
    pub mod message {
        pub const ECHO_REPLY: u8 = 0;
        pub const DST_UNREACHABLE: u8 = 3;
        pub const REDIRECT: u8 = 5;
        pub const ECHO_REQUEST: u8 = 8;
        pub const ROUTER_ADVERT: u8 = 9;
        pub const ROUTER_SOLICIT: u8 = 10;
        pub const TIME_EXCEEDED: u8 = 11;
        pub const PARAK_PROBLEM: u8 = 12;
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
}

/// Internet protocol control message type.
pub enum Message {
    /// Echo request
    EchoRequest,
    /// Echo reply
    EchoReply,
    /// Destination unreachable
    DstUnreachable,
    /// Message redirect
    Redirect,
    /// Router advertisement
    RouterAdvert,
    /// Router solicitation
    RouterSolicit,
    /// Time exceeded
    TimeExceeded,
    /// Parameter problem
    ParamProblem,
    /// Timestamp
    Timestamp,
    /// Timestamp reply
    TimestampReply,
}

pub enum DstUnreachable {
    /// Destination network unreachable
    NetUnreachable,
    /// Destination host unreachable
    HostUnreachable,
    /// Destination protocol unreachable
    ProtoUnreachable,
    /// Destination port unreachable
    PortUnreachable ,
    /// Fragmentation required, and DF flag set
    FragRequired,
    /// Source route failed
    SrcRouteFailed,
    /// Destination network unknown
    DstNetUnknown ,
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
}
