use npcap_rs::{Device, TCPPacket};

struct Options {
    pub host: String,
    pub port: u16,
}

fn main() {
    let pcap = npcap_rs::PCap::new().unwrap();
    let devices: Vec<Device> = pcap
        .active_devices()
        .into_iter()
        .filter(|d| {
            d.maybe_address()
                .map(|addr| addr.is_ipv4())
                .unwrap_or(false)
        })
        .collect();

    // TODO: We need select the NetCard device by the given ip address instead of hardcode.
    let dev = pcap
        .find_device("Ethernet Controller")
        .expect("unable to find the netcard device");
    println!("use device: {:?}, {:?}", dev.name, dev.desc);

    let (listener, receiver) = dev.open(None).expect("open device failed");

    // Only accept the tcp packet send to the given redis
    // and tcp dst port $PORT
    if !listener.set_filter(&dev, "(ip dst host 192.168.3.8) and (tcp)") {
        panic!("failed to set_filter to the npcap for capture packet.");
    }

    listener.run();

    loop {
        let pkt = receiver.recv().expect("recv packet from npcap failed");
        match pkt.tcp {
            None => {
                unreachable!("already filter the tcp packet");
            }
            Some(ref tcp_pkt) => {
                let usr_data = &tcp_pkt.data;
                print!(
                    "From {}:{} ",
                    pkt.ip_hdr.source_addr, tcp_pkt.hdr.source_port
                );
                match usr_data {
                    npcap_rs::TCPApps::Generic(app_data) => {
                        let resp_body = app_data.as_ref();
                        match resp_body {
                            None => {
                                // Maybe some tcp control packet without userData, we skip it.
                                // println!("skip tcp packet without any user data.")
                            }
                            Some(resp) => {
                                print!("{:?}", resp)
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        println!("");
    }
}
