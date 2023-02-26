//! List all devices.
//!

use std::io::{self, Write};

use npcap_rs::Device;

fn main() {
    let pcap = npcap_rs::PCap::new().unwrap();
    let devs: Vec<Device> = pcap.active_devices();
    for (idx, dev) in devs.iter().enumerate() {
        println!(
            "{}: {:?} '{:?}' {:#04X?}",
            idx, dev.name, dev.desc, dev.flags,
        );
    }

    print!("Select an interface: ");
    io::stdout().flush().unwrap();

    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("give me your choise!");

    let sel = inp.trim().parse::<u8>().unwrap();
    println!("Selected: {}", sel);
    if let Some((listener, rx)) = devs[sel as usize].open(None) {
        listener.set_filter(&devs[sel as usize], "ip and tcp");
        listener.run();

        while let Ok(pack) = rx.recv() {
            println!("{:?}", pack);
        }
    }
}
