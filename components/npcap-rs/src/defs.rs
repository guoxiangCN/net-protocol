// About the Device::flags
// #define PCAP_IF_LOOPBACK				0x00000001	/* interface is loopback */
// #define PCAP_IF_UP					0x00000002	/* interface is up */
// #define PCAP_IF_RUNNING					0x00000004	/* interface is running */
// #define PCAP_IF_WIRELESS				0x00000008	/* interface is wireless (*NOT* necessarily Wi-Fi!) */
// #define PCAP_IF_CONNECTION_STATUS			0x00000030	/* connection status: */
// #define PCAP_IF_CONNECTION_STATUS_UNKNOWN		0x00000000	/* unknown */
// #define PCAP_IF_CONNECTION_STATUS_CONNECTED		0x00000010	/* connected */
// #define PCAP_IF_CONNECTION_STATUS_DISCONNECTED		0x00000020	/* disconnected */
// #define PCAP_IF_CONNECTION_STATUS_NOT_APPLICABLE	0x00000030	/* not applicable */
pub const PCAP_IF_LOOPBACK: u32 = 0x00000001; /* interface is loopback */
pub const PCAP_IF_UP: u32 = 0x00000002; /* interface is up */
pub const PCAP_IF_RUNNING: u32 = 0x00000004; /* interface is running */

//
// Although AF_UNSPEC is defined for backwards compatibility, using
// AF_UNSPEC for the "af" parameter when creating a socket is STRONGLY
// DISCOURAGED.  The interpretation of the "protocol" parameter
// depends on the actual address family chosen.  As environments grow
// to include more and more address families that use overlapping
// protocol values there is more and more chance of choosing an
// undesired address family when AF_UNSPEC is used.
//

pub const AF_UNIX: u16 = 1; // local to host (pipes, portals)
pub const AF_INET: u16 = 2; // internetwork: UDP, TCP, etc.
pub const AF_INET6: u16 = 23; // Internetwork Version 6

// #define AF_UNSPEC       0               // unspecified
// #define AF_UNIX         1               // local to host (pipes, portals)
// #define AF_INET         2               // internetwork: UDP, TCP, etc.
// #define AF_IMPLINK      3               // arpanet imp addresses
// #define AF_PUP          4               // pup protocols: e.g. BSP
// #define AF_CHAOS        5               // mit CHAOS protocols
// #define AF_NS           6               // XEROX NS protocols
// #define AF_IPX          AF_NS           // IPX protocols: IPX, SPX, etc.
// #define AF_ISO          7               // ISO protocols
// #define AF_OSI          AF_ISO          // OSI is ISO
// #define AF_ECMA         8               // european computer manufacturers
// #define AF_DATAKIT      9               // datakit protocols
// #define AF_CCITT        10              // CCITT protocols, X.25 etc
// #define AF_SNA          11              // IBM SNA
// #define AF_DECnet       12              // DECnet
// #define AF_DLI          13              // Direct data link interface
// #define AF_LAT          14              // LAT
// #define AF_HYLINK       15              // NSC Hyperchannel
// #define AF_APPLETALK    16              // AppleTalk
// #define AF_NETBIOS      17              // NetBios-style addresses
// #define AF_VOICEVIEW    18              // VoiceView
// #define AF_FIREFOX      19              // Protocols from Firefox
// #define AF_UNKNOWN1     20              // Somebody is using this!
// #define AF_BAN          21              // Banyan
// #define AF_ATM          22              // Native ATM Services
// #define AF_INET6        23              // Internetwork Version 6
// #define AF_CLUSTER      24              // Microsoft Wolfpack
// #define AF_12844        25              // IEEE 1284.4 WG AF
// #define AF_IRDA         26              // IrDA
// #define AF_NETDES       28              // Network Designers OSI & gateway
