use pktparse::ethernet::{self, EtherType};

use crate::{ApplicationProtocol, Packet, TCPPacket};

macro_rules! clone_data {
    ($slice: expr) => {
        if !$slice.is_empty() {
            Some($slice.to_vec())
        } else {
            None
        }
    };
}

/// Parse the raw packet.
///
/// ## Notes
///
/// 1. Every packet lives in an Ethernet frame.
/// 2. MTU may restrict the size of packet. UDP packet can be as big as 2^16-1 bytes (65535 bytes)
///    but the Ethernet frame can only contain 1500 bytes of data. Larger UDP packets will get
///    defragmented.
///
/// ## References:
/// - https://jvns.ca/blog/2017/02/07/mtu/
pub fn parse_raw(data: &[u8]) -> Option<crate::Packet> {
    if let Ok((remaining, eth_frame)) = ethernet::parse_ethernet_frame(data) {
        let etype = eth_frame.ethertype;
        if etype == EtherType::IPv4 {
            if let Ok((remaining, header)) = pktparse::ipv4::parse_ipv4_header(remaining) {
                let mut packet = Packet {
                    ether_hdr: eth_frame,
                    ip_hdr: header,
                    // assume it's tcp
                    app_prot: crate::ApplicationProtocol::TCP,
                    tcp: None,
                    udp: None,
                };
                match header.protocol {
                    pktparse::ip::IPProtocol::TCP => {
                        if let Ok((remaining, hdr)) = pktparse::tcp::parse_tcp_header(remaining) {
                            #[cfg(feature = "http-parse")]
                            let mut headers_buffer = vec![http_bytes::EMPTY_HEADER; 20];

                            let mut pack = TCPPacket {
                                hdr,
                                data: crate::TCPApps::Generic(None),
                            };
                            #[cfg(feature = "http-parse")]
                            {
                                if let Ok((http_header)) = http_bytes::parse_request_header(
                                    remaining,
                                    &mut headers_buffer,
                                    Some(http_bytes::http::uri::Scheme::HTTP),
                                ) {
                                    if let Some((req, remain)) = http_header {
                                        pack.data = crate::TCPApps::HTTP(req);
                                    }
                                } else {
                                    let data = clone_data!(remaining);
                                    pack.data = crate::TCPApps::Generic(data);
                                }
                            }
                            #[cfg(not(feature = "http-parse"))]
                            {
                                let data = clone_data!(remaining);
                                pack.data = crate::TCPApps::Generic(data);
                            }
                            packet.tcp = Some(pack);
                        }
                    }
                    pktparse::ip::IPProtocol::UDP => {
                        if let Ok((remaining, hdr)) = pktparse::udp::parse_udp_header(remaining) {
                            let data = if !remaining.is_empty() {
                                #[cfg(feature = "dns-parse")]
                                {
                                    if let Ok(dns) = dns_parser::Packet::parse(remaining) {
                                        crate::UDPApp::DNS(crate::dns::from_packet(&dns))
                                    } else {
                                        crate::UDPApp::Generic(Some(remaining.to_vec()))
                                    }
                                }
                                #[cfg(not(feature = "dns-parse"))]
                                {
                                    // might be expensive `.to_vec` call
                                    crate::UDPApp::Generic(Some(remaining.to_vec()))
                                }
                            } else {
                                crate::UDPApp::Generic(None)
                            };

                            let pack = crate::UDPPacket { hdr, data };
                            packet.app_prot = ApplicationProtocol::UDP;
                            packet.udp = Some(pack);
                        }
                    }
                    _ => {
                        return None;
                        //unimplemented!()
                    }
                }
                return Some(packet);
            }
        } else {
            eprintln!(" - Unsupported Ethernet frame type: {:?}", etype);
        }
        return None;
    }
    None
}

use crossbeam_channel::{bounded, unbounded, Receiver, Sender};

pub type Rx<T> = Receiver<T>;
pub type Tx<T> = Sender<T>;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    unbounded()
}

pub fn channel_bound<T>(size: usize) -> (Sender<T>, Receiver<T>) {
    bounded(size)
}
