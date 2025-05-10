use super::parser;
use std::collections::HashMap;

pub struct Reassembler {
    store: HashMap<u32, Vec<u8>>,
}

impl Reassembler {
    pub fn new() -> Self {
        Reassembler {
            store: std::collections::HashMap::new(),
        }
    }

    pub fn add_and_check(&self, payload: &[u8]) {
        let h = parser::parse(payload);
        if h.dont_fragment || !h.more_fragment {
            println!("{:?}", h);
        }
    }
}
