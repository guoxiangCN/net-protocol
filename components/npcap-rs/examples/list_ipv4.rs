//! List all devices.
//!

use npcap_rs::Device;

fn main() {
    let pcap = npcap_rs::PCap::new().unwrap();
    let devs: Vec<Device> = pcap
        .active_devices()
        .into_iter()
        .filter(|dev| {
            dev.maybe_address()
                .map(|addr| addr.is_ipv4())
                .unwrap_or(false)
        })
        .collect();
    for (idx, dev) in devs.iter().enumerate() {
        println!(
            "{}: {:?} '{:?}' {:#04X?}",
            idx, dev.name, dev.desc, dev.flags,
        );
    }
}
