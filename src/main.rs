use mac_address::format;
use pnet::datalink;

mod mac_address;

fn main() {
    use pnet::datalink::Channel::Ethernet;

    let binding = datalink::interfaces();
    let interface = binding
        .iter()
        .filter(|x| x.is_up())
        .filter(|x| !x.is_loopback())
        .next()
        .expect("no up interface");
    println!("{:?}", interface);

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("unknown channel type"),
        Err(_) => panic!("failed to open channel"),
    };
    loop {
        let ethernet_frame = rx.next();
        match ethernet_frame {
            Ok(frame) => {
                let dest_mac_address = &frame[0..6];
                let src_mac_address = &frame[6..12];
                println!(
                    "Destination MAC: {}, Source MAC: {}",
                    format::byte_array_2_hex(dest_mac_address),
                    format::byte_array_2_hex(src_mac_address)
                )
            }
            Err(_) => return,
        }
    }
}
