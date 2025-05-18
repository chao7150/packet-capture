use super::header::{Header, Protocol};
use std::net::IpAddr;

pub fn parse(payload: &[u8]) -> (Header, &[u8]) {
    if payload.len() < 20 {
        panic!("Payload is too short to parse");
    }
    let header = Header {
        id: u16::from_be_bytes([payload[4], payload[5]]),
        dont_fragment: payload[6] & 0x40 != 0,
        more_fragment: payload[6] & 0x20 != 0,
        fragment_offset: ((payload[6] as u16 & 0x1F) << 8) | payload[7] as u16,
        source_ip: IpAddr::from([payload[12], payload[13], payload[14], payload[15]]),
        destination_ip: IpAddr::from([payload[16], payload[17], payload[18], payload[19]]),
        protocol: parse_protocol(&payload[9]),
    };
    (header, &payload[20..])
}

fn parse_protocol(payload: &u8) -> Protocol {
    match payload {
        6 => Protocol::TCP,
        17 => Protocol::UDP,
        _ => Protocol::Others,
    }
}
