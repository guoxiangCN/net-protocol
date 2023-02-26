#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum PcapErrCode {
    PCAP_ERROR = 0,
    PCAP_ERROR_BREAK = -2,
    PCAP_ERROR_NOT_ACTIVATED = -3,
}
