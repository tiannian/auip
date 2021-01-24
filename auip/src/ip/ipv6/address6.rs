use core::fmt::{self, Debug};
use byteorder::{ByteOrder, NetworkEndian};

#[derive(Clone)]
pub struct Address(pub [u8; 16]);

impl Default for Address {
    fn default() -> Self {
        Self([0; 16])
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_ipv4_mapped() {
            return write!(f, "::ffff:{}.{}.{}.{}", self.0[12], self.0[13], self.0[14], self.0[15])
        }

        // The string representation of an IPv6 address should
        // collapse a series of 16 bit sections that evaluate
        // to 0 to "::"
        //
        // See https://tools.ietf.org/html/rfc4291#section-2.2
        // for details.
        enum State {
            Head,
            HeadBody,
            Tail,
            TailBody
        }
        let mut words = [0u16; 8];
        self.write_parts(&mut words);
        let mut state = State::Head;
        for word in words.iter() {
            state = match (*word, &state) {
                // Once a u16 equal to zero write a double colon and
                // skip to the next non-zero u16.
                (0, &State::Head) | (0, &State::HeadBody) => {
                    write!(f, "::")?;
                    State::Tail
                },
                // Continue iterating without writing any characters until
                // we hit anothing non-zero value.
                (0, &State::Tail) => State::Tail,
                // When the state is Head or Tail write a u16 in hexadecimal
                // without the leading colon if the value is not 0.
                (_, &State::Head) => {
                    write!(f, "{:x}", word)?;
                    State::HeadBody
                },
                (_, &State::Tail) => {
                    write!(f, "{:x}", word)?;
                    State::TailBody
                },
                // Write the u16 with a leading colon when parsing a value
                // that isn't the first in a section
                (_, &State::HeadBody) | (_, &State::TailBody) => {
                    write!(f, ":{:x}", word)?;
                    state
                }
            }
        }
        Ok(())
    }
}

impl From<[u8; 16]> for Address {
    fn from(v: [u8; 16]) -> Self {
        Self(v)
    }
}

impl From<&[u8]> for Address {
    fn from(v: &[u8]) -> Self {
        let mut bytes = [0u8; 16];
        bytes.copy_from_slice(v);
        Self(bytes)
    }
}

impl Address {
    pub fn new(a0: u16, a1: u16, a2: u16, a3: u16,
               a4: u16, a5: u16, a6: u16, a7: u16) -> Address {
        let mut addr = [0u8; 16];
        NetworkEndian::write_u16(&mut addr[0..2], a0);
        NetworkEndian::write_u16(&mut addr[2..4], a1);
        NetworkEndian::write_u16(&mut addr[4..6], a2);
        NetworkEndian::write_u16(&mut addr[6..8], a3);
        NetworkEndian::write_u16(&mut addr[8..10], a4);
        NetworkEndian::write_u16(&mut addr[10..12], a5);
        NetworkEndian::write_u16(&mut addr[12..14], a6);
        NetworkEndian::write_u16(&mut addr[14..16], a7);
        Address(addr)
    }

    pub fn write_parts(&self, data: &mut [u16]) {
        assert!(data.len() >= 8);
        for (i, chunk) in self.0.chunks(2).enumerate() {
            data[i] = NetworkEndian::read_u16(chunk);
        }
    }

    pub fn is_ipv4_mapped(&self) -> bool {
        self.0[0..12] == [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff]
    }
}
