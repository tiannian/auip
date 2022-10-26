use byteorder::{ByteOrder, NetworkEndian};

use crate::{layer3::Address, Error, Result};

fn propagate_carries(word: u32) -> u16 {
    let sum = (word >> 16) + (word & 0xffff);
    ((sum >> 16) as u16) + (sum as u16)
}

/// Compute an RFC 1071 compliant checksum (without the final complement).
pub fn data(mut data: &[u8]) -> u16 {
    let mut accum = 0;

    // For each 32-byte chunk...
    const CHUNK_SIZE: usize = 32;
    while data.len() >= CHUNK_SIZE {
        let mut d = &data[..CHUNK_SIZE];
        // ... take by 2 bytes and sum them.
        while d.len() >= 2 {
            accum += NetworkEndian::read_u16(d) as u32;
            d = &d[2..];
        }

        data = &data[CHUNK_SIZE..];
    }

    // Sum the rest that does not fit the last 32-byte chunk,
    // taking by 2 bytes.
    while data.len() >= 2 {
        accum += NetworkEndian::read_u16(data) as u32;
        data = &data[2..];
    }

    // Add the last remaining odd byte, if any.
    if let Some(&value) = data.first() {
        accum += (value as u32) << 8;
    }

    propagate_carries(accum)
}

/// Combine several RFC 1071 compliant checksums.
pub fn combine(checksums: &[u16]) -> u16 {
    let mut accum: u32 = 0;
    for &word in checksums {
        accum += word as u32;
    }
    propagate_carries(accum)
}

/// Compute an IP pseudo header checksum.
pub fn pseudo_ip_header(
    src_addr: &Address,
    dst_addr: &Address,
    next_header: u8,
    length: u32,
) -> Result<u16> {
    match (src_addr, dst_addr) {
        (&Address::Ipv4(src_addr), &Address::Ipv4(dst_addr)) => {
            let mut proto_len = [0u8; 4];
            proto_len[1] = next_header;
            NetworkEndian::write_u16(&mut proto_len[2..4], length as u16);

            Ok(combine(&[
                data(src_addr.as_bytes()),
                data(dst_addr.as_bytes()),
                data(&proto_len[..]),
            ]))
        }
        /*         (&Address::Ipv6(src_addr), &Address::Ipv6(dst_addr)) => { */
        /* let mut proto_len = [0u8; 8]; */
        /* proto_len[7] = next_header.into(); */
        /* NetworkEndian::write_u32(&mut proto_len[0..4], length); */
        /* combine(&[ */
        /*     data(src_addr.as_bytes()), */
        /*     data(dst_addr.as_bytes()), */
        /*     data(&proto_len[..]), */
        /* ]) */
        /* } */
        _ => Err(Error::SrcAndDstMustSame),
    }
}
