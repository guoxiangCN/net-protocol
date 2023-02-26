fn main() {
    println!("Libpcap version: {}", npcap_rs::version());
    let devs = npcap_rs::PCap::new().unwrap();

    let dev = devs
        .devices()
        .find(|dev| dev.desc.as_ref().unwrap() == "Intel(R) Ethernet Controller (3) I225-V");

    if let Some(dev) = dev {
        let (listener, _) = dev.open(None).unwrap();

        while let Some(pack) = listener.next_packet() {
            println!("{:?}", pack);
        }
    }
}
