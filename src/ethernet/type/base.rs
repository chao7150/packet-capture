use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Type {
    IPv4,
    IPv6,
    ARP,
    Others,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::IPv4 => write!(f, "IPv4"),
            Type::IPv6 => write!(f, "IPv6"),
            Type::ARP => write!(f, "ARP"),
            Type::Others => write!(f, "Other Protocol"),
        }
    }
}
