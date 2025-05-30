use pnet::datalink;

mod ethernet;
mod network;

pub struct Config {}

pub fn run() {
    use pnet::datalink::Channel::Ethernet;

    let binding = datalink::interfaces();
    let interface = binding
        .iter()
        .find(|x| x.is_up() && !x.is_loopback())
        .expect("no up interface");
    println!("{:?}", interface);

    let (_, mut rx) = match datalink::channel(interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("unknown channel type"),
        Err(_) => panic!("failed to open channel"),
    };
    let mut reassembler = network::ipv4::reassembler::Assorter::new();
    loop {
        let ethernet_frame = rx.next();
        match ethernet_frame {
            Ok(frame) => {
                let dest_mac_address = &frame[0..6];
                let src_mac_address = &frame[6..12];
                let frame_type = &frame[12..14];
                println!(
                    "Destination MAC: {}, Source MAC: {}, Frame Type: {}",
                    ethernet::mac_address::format::byte_array_2_hex(dest_mac_address),
                    ethernet::mac_address::format::byte_array_2_hex(src_mac_address),
                    ethernet::r#type::format::from_byte_array(frame_type),
                );
                let payload = &frame[14..];
                if let Some(reassembled) = reassembler.add_and_check(payload) {
                    println!("reassembled header: {:?}", reassembled.0);
                }
            }
            Err(_) => return,
        }
    }
}
