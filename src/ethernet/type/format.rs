use super::base::Type;

pub fn from_byte_array(bytes: &[u8]) -> Type {
    let type_value = ((bytes[0] as u16) << 8) | (bytes[1] as u16);

    match type_value {
        0x0800 => Type::IPv4,
        0x0806 => Type::ARP,
        0x86DD => Type::IPv6,
        _ => Type::Others,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_byte_array() {
        let bytes = [0x08, 0x00];
        assert_eq!(from_byte_array(&bytes), Type::IPv4);
        let bytes_arp = [0x08, 0x06];
        assert_eq!(from_byte_array(&bytes_arp), Type::ARP);
        let bytes_ipv6 = [0x86, 0xDD];
        assert_eq!(from_byte_array(&bytes_ipv6), Type::IPv6);
    }
}
