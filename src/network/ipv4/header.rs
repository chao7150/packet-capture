use std::net::IpAddr;

#[derive(Debug)]
pub struct Header {
    pub id: u16,
    pub dont_fragment: bool,
    pub more_fragment: bool,
    pub fragment_offset: u16,
    #[allow(dead_code)]
    pub source_ip: IpAddr,
    #[allow(dead_code)]
    pub destination_ip: IpAddr,
    #[allow(dead_code)]
    pub protocol: Protocol,
}

impl Header {
    pub fn is_last_fragment(&self) -> bool {
        self.dont_fragment || !self.more_fragment
    }
}

#[derive(Debug)]
pub enum Protocol {
    Tcp,
    Udp,
    Others,
}
