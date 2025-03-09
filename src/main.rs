use pnet::datalink;

fn main() {
    let interface = datalink::interfaces()
        .into_iter()
        .filter(|x| x.is_up())
        .filter(|x| !x.is_loopback())
        .next()
        .expect("no up interface");
    println!("{:?}", interface);
}
