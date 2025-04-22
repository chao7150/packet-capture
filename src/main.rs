use pnet::datalink;

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
        println!("{:?}", rx.next());
    }
}
