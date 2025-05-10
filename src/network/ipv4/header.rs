use std::net::IpAddr;

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub dont_fragment: bool,
    pub more_fragment: bool,
    pub fragment_offset: u16,
    pub source_ip: IpAddr,
    pub destination_ip: IpAddr,
    pub protocol: Protocol,
}

#[derive(Debug)]
pub enum Protocol {
    TCP,
    UDP,
    Others,
}
