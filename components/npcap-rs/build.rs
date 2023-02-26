
#[cfg(target_os = "windows")]
use std::env;

fn main() {
    #[cfg(target_os = "windows")]
    {
        env::set_var("NPCAP_RS_LIB_DIR", "D:\\npcap-sdk-1.13\\Lib\\x64");

        let path = env::var("NPCAP_RS_LIB_DIR");
        if let Ok(path) = path {
            println!("cargo:rustc-link-search=all={}", path);
            println!("cargo:rustc-link-lib=static=wpcap");
        } else {
            panic!("Couldn't find the path to npcap");
        }
    }

    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=dylib=pcap");
    }
}
