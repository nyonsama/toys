use chrono::{DateTime, Local};
use std::{collections::HashSet, sync::mpsc};

use pnet::{
    datalink,
    packet::{
        arp::{ArpOperations, ArpPacket},
        ethernet::{EtherTypes, EthernetPacket},
        icmp::{IcmpPacket, IcmpTypes},
        icmpv6::{Icmpv6Packet, Icmpv6Types},
        ip::IpNextHeaderProtocol,
        ipv4::Ipv4Packet,
        ipv6::Ipv6Packet,
        tcp::{TcpOptionNumbers, TcpPacket},
        udp::UdpPacket,
        Packet,
    },
    util::MacAddr,
};
use pretty_hex::*;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "rcaptor")]
struct Opt {
    #[structopt(
        short,
        long,
        help = "Filter specified protocol. Available protocols: arp, ip, ipv6, icmp, icmpv6, tcp, udp, other"
    )]
    protocol: Option<Vec<String>>,

    #[structopt(short, long, help = "Name of the interface that will be captured")]
    ifname: String,

    #[structopt(short = "x", long, help = "Output packet content in hexdump format")]
    hexdump: bool,

    // #[structopt(short,long,help="")]
    // filter: Vec<String>,
    #[structopt(short, long, help = "Print all available information")]
    all: bool,

    #[structopt(short, long, help = "Show Ethernet header")]
    ethernet: bool,
}

#[derive(Debug, Clone)]
struct Statistics {
    start_time: DateTime<Local>,
    stop_time: DateTime<Local>,
    l2_count: usize,
    l2_broadcast: usize,
    l2_short: usize,
    l2_long: usize,
    l2_bytes: usize,
    ipv4_count: usize,
    ipv4_broadcast: usize,
    ipv4_bytes: usize,
    ipv6_count: usize,
    ipv6_bytes: usize,
    tcp_count: usize,
    udp_count: usize,
    icmp_count: usize,
    icmp_redirect: usize,
    icmp_dest_unreachable: usize,
    icmpv6_count: usize,
    icmpv6_redirect: usize,
    icmpv6_dest_unreachable: usize,
    arp_count: usize,
}
impl Statistics {
    pub fn new() -> Statistics {
        Statistics {
            start_time: Local::now(),
            stop_time: Local::now(),
            l2_count: 0,
            l2_broadcast: 0,
            l2_short: 0,
            l2_long: 0,
            l2_bytes: 0,
            ipv4_count: 0,
            ipv4_broadcast: 0,
            ipv4_bytes: 0,
            ipv6_count: 0,
            ipv6_bytes: 0,
            tcp_count: 0,
            udp_count: 0,
            icmp_count: 0,
            icmp_redirect: 0,
            icmp_dest_unreachable: 0,
            icmpv6_count: 0,
            icmpv6_redirect: 0,
            icmpv6_dest_unreachable: 0,
            arp_count: 0,
        }
    }
    pub fn stop(&mut self) {
        self.stop_time = Local::now();
    }
}

#[derive(Debug, Clone, Default)]
struct PacketFilter {
    eth: bool,
    tcp: bool,
    udp: bool,
    icmp: bool,
    icmpv6: bool,
    ipv4: bool,
    ipv6: bool,
    arp: bool,
    other: bool,
    payload: bool,
}
#[derive(Debug, Clone)]
struct Parser {
    filter: PacketFilter,
    statistics: Statistics,
    known_layer2_hosts: HashSet<MacAddr>,
}

impl Parser {
    pub fn new(filter: PacketFilter) -> Parser {
        Parser {
            filter,
            statistics: Statistics::new(),
            known_layer2_hosts: HashSet::new(),
        }
    }

    /// packet is raw layer 2 packet
    pub fn parse(&mut self, packet: &[u8]) {
        info("packet start");
        let t = EthernetPacket::new(packet);
        if let Some(t) = t {
            self.statistics.l2_count += 1;
            self.statistics.l2_bytes += packet.len();
            match packet.len() {
                0..=63 => self.statistics.l2_short += 1,
                1519.. => self.statistics.l2_long += 1,
                _ => (),
            }
            if t.get_destination().is_broadcast() {
                self.statistics.l2_broadcast += 1;
            }
            if self.filter.eth {
                println!("+{:-^60}", "Layer2");
                println!(
                    "|from: {}    to: {}    length: {}",
                    t.get_source().to_string(),
                    t.get_destination().to_string(),
                    packet.len()
                );
            }
            // println!("|length:{}", packet.len());
            match t.get_ethertype() {
                EtherTypes::Ipv4 => self.parse_ipv4(t.payload()),
                EtherTypes::Ipv6 => self.parse_ipv6(t.payload()),
                EtherTypes::Arp => self.parse_arp(t.payload()),
                _ => {
                    if self.filter.other {
                        error("This protocol is not implemented")
                    }
                    if self.filter.payload {
                        println!("{:?}", t.payload().hex_dump());
                    }
                }
            }
            if self.known_layer2_hosts.insert(t.get_source()) {
                info(&format!("new device found: {}", t.get_source().to_string()));
            }
        } else {
            error("Malformed layer2 packet")
        }
    }
    fn parse_tcp(&mut self, packet: &[u8]) {
        if !self.filter.tcp {
            return;
        }
        let tcp = TcpPacket::new(packet);
        println!("+{:-^60}", "Layer4: TCP");
        if let Some(tcp) = tcp {
            self.statistics.tcp_count += 1;
            println!(
                "|source port: {}    destination port: {}",
                tcp.get_source(),
                tcp.get_destination()
            );
            println!("|length: {}", packet.len());
            println!("|flags: {:#018b}", tcp.get_flags());
            println!(
                "|options: {:?}",
                tcp.get_options_iter()
                    .map(|s| match s.get_number() {
                        TcpOptionNumbers::EOL => "EOL",
                        TcpOptionNumbers::NOP => "NOP",
                        TcpOptionNumbers::MSS => "MSS",
                        TcpOptionNumbers::WSCALE => "WSCALE",
                        TcpOptionNumbers::SACK_PERMITTED => "SACK_PERMITTED",
                        TcpOptionNumbers::SACK => "SACK",
                        TcpOptionNumbers::TIMESTAMPS => "TIMESTAMPS",
                        _ => panic!("unreachable"),
                    })
                    .collect::<Vec<&str>>()
            );
            if self.filter.payload {
                println!("{:?}", tcp.payload().hex_dump());
            }
        } else {
            error("Malformed TCP packet");
        }
    }
    fn parse_udp(&mut self, packet: &[u8]) {
        if !self.filter.udp {
            return;
        }
        let udp = UdpPacket::new(packet);
        println!("+{:-^60}", "Layer4: UDP");
        if let Some(udp) = udp {
            self.statistics.udp_count += 1;
            println!(
                "|source port: {}    destination port: {}",
                udp.get_source(),
                udp.get_destination()
            );
            println!("|length: {}", packet.len());
            if self.filter.payload {
                println!("{:?}", udp.payload().hex_dump());
            }
        }
    }
    fn parse_icmp(&mut self, packet: &[u8]) {
        if !self.filter.icmp {
            return;
        }
        let icmp = IcmpPacket::new(packet);
        if let Some(icmp) = icmp {
            self.statistics.icmp_count += 1;
            println!("+{:-^60}", "Layer4: ICMP");
            println!(
                "|type: {}    code: {}",
                icmp.get_icmp_type().0,
                icmp.get_icmp_code().0
            );
            match icmp.get_icmp_type() {
                IcmpTypes::DestinationUnreachable => self.statistics.icmp_dest_unreachable += 1,
                IcmpTypes::RedirectMessage => self.statistics.icmp_redirect += 1,
                _ => (),
            }
            println!("|length: {}", packet.len());
            if self.filter.payload {
                println!("{:?}", icmp.payload().hex_dump());
            }
        }
    }
    fn parse_icmpv6(&mut self, packet: &[u8]) {
        if !self.filter.icmpv6 {
            return;
        }
        let icmpv6 = Icmpv6Packet::new(packet);
        if let Some(icmpv6) = icmpv6 {
            self.statistics.icmpv6_count += 1;
            println!("+{:-^60}", "Layer4: ICMPv6");
            println!(
                "|type: {}    code: {}",
                icmpv6.get_icmpv6_type().0,
                icmpv6.get_icmpv6_code().0
            );
            match icmpv6.get_icmpv6_type() {
                Icmpv6Types::DestinationUnreachable => self.statistics.icmpv6_dest_unreachable += 1,
                Icmpv6Types::Redirect => self.statistics.icmpv6_redirect += 1,
                _ => (),
            }
            println!("|length: {}", packet.len());
            if self.filter.payload {
                println!("{:?}", icmpv6.payload().hex_dump());
            }
        }
    }

    fn parse_layer4(&mut self, protocol: IpNextHeaderProtocol, packet: &[u8]) {
        use pnet::packet::ip::IpNextHeaderProtocols;
        match protocol {
            IpNextHeaderProtocols::Tcp => self.parse_tcp(packet),
            IpNextHeaderProtocols::Udp => self.parse_udp(packet),
            IpNextHeaderProtocols::Icmp => self.parse_icmp(packet),
            IpNextHeaderProtocols::Icmpv6 => self.parse_icmpv6(packet),
            _ => {
                if self.filter.other {
                    error("This protocol is not implemented")
                }
                if self.filter.payload {
                    println!("{:?}", packet.hex_dump())
                }
            }
        }
    }
    fn parse_ipv6(&mut self, packet: &[u8]) {
        if self.filter.ipv6 {
            println!("+{:-^60}", "Layer3: IPv6");
        }
        let ipv6 = Ipv6Packet::new(packet);
        if let Some(ipv6) = ipv6 {
            if self.filter.ipv6 {
                println!(
                    "|from: {}    to: {}",
                    ipv6.get_source().to_string(),
                    ipv6.get_destination().to_string()
                );
                println!("|length: {}", ipv6.get_payload_length() + 40);
                // info("Found ipv6 packet");
                self.statistics.ipv6_count += 1;
                self.statistics.ipv6_bytes += packet.len();
                self.parse_layer4(ipv6.get_next_header(), ipv6.payload());
            }
        } else {
            warn("Malformed IPv6 packet");
        }
    }
    fn parse_ipv4(&mut self, packet: &[u8]) {
        if self.filter.ipv4 {
            println!("+{:-^60}", "Layer3: IPv4");
        }
        let ipv4 = Ipv4Packet::new(packet);
        if let Some(ipv4) = ipv4 {
            // info("Found ipv4 packet");
            if self.filter.ipv4 {
                println!(
                    "|from: {}    to: {}",
                    ipv4.get_source().to_string(),
                    ipv4.get_destination().to_string()
                );
                println!("|length: {}", ipv4.get_total_length());
                println!("|dscp: {:#08b}", ipv4.get_dscp());
                println!("|ecn:  {:#04b}", ipv4.get_ecn());
                println!("|flags: {:#05b}", ipv4.get_flags());
                println!("|options: {:?}", ipv4.get_options());
            }
            self.statistics.ipv4_count += 1;
            self.statistics.ipv4_bytes += packet.len();
            if ipv4.get_destination().is_broadcast() {
                self.statistics.ipv4_broadcast += 1;
            }
            self.parse_layer4(ipv4.get_next_level_protocol(), ipv4.payload());
        } else {
            warn("Malformed IPv4 packet");
        }
    }
    fn parse_arp(&mut self, packet: &[u8]) {
        if !self.filter.arp {
            return;
        }
        println!("+{:-^60}", "Layer3: ARP");
        let arp = ArpPacket::new(packet);
        if let Some(arp) = arp {
            // info("Found ARP packet");
            println!(
                "|from: {}    to: {}",
                arp.get_sender_proto_addr().to_string(),
                arp.get_target_proto_addr().to_string()
            );
            println!(
                "|operation: {}",
                match arp.get_operation() {
                    ArpOperations::Request => "Request",
                    ArpOperations::Reply => "Reply",
                    _ => panic!("unreachable"),
                }
            );
            self.statistics.arp_count += 1;
            if self.filter.payload {
                println!("{:?}", arp.payload().hex_dump())
            }
        } else {
            warn("Malformed ARP Packet");
        }
    }
    pub fn print_summary(&mut self) {
        fn print_barrier() {
            println!("+{:-^30}+{:-^30}+", "", "");
        }
        fn print_kv(k: &str, v: &str) {
            println!("|{: ^30}|{: ^30}|", k, v);
        }
        self.statistics.stop();
        let seconds = self
            .statistics
            .stop_time
            .signed_duration_since(self.statistics.start_time)
            .num_seconds()
            .abs() as usize;
        println!("+{:-^61}+", "");
        println!("|{: ^61}|", "Summary");
        print_barrier();
        print_kv(
            "Start time",
            &self
                .statistics
                .start_time
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        );
        print_barrier();
        print_kv(
            "Stop time",
            &self
                .statistics
                .stop_time
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
        );
        print_barrier();
        print_kv(
            "MAC broadcast",
            &format!("{}", self.statistics.l2_broadcast),
        );
        print_barrier();
        print_kv("MAC short", &format!("{}", self.statistics.l2_short));
        print_barrier();
        print_kv("MAC long", &format!("{}", self.statistics.l2_long));
        print_barrier();
        print_kv("MAC bytes", &format!("{}", self.statistics.l2_bytes));
        print_barrier();
        print_kv("MAC packet count", &format!("{}", self.statistics.l2_count));
        print_barrier();
        print_kv(
            "bit/s",
            &format!("{}", self.statistics.l2_bytes * 8 / seconds),
        );
        print_barrier();
        print_kv("byte/s", &format!("{}", self.statistics.l2_bytes / seconds));
        print_barrier();
        print_kv(
            "packet/s",
            &format!("{}", self.statistics.l2_count / seconds),
        );
        print_barrier();
        print_kv(
            "IPv4 broadcast",
            &format!("{}", self.statistics.ipv4_broadcast),
        );
        print_barrier();
        print_kv("IPv4 bytes", &format!("{}", self.statistics.ipv4_bytes));
        print_barrier();
        print_kv(
            "IPv4 packet count",
            &format!("{}", self.statistics.ipv4_count),
        );
        print_barrier();
        print_kv("IPv6 bytes", &format!("{}", self.statistics.ipv6_bytes));
        print_barrier();
        print_kv(
            "IPv6 packet count",
            &format!("{}", self.statistics.ipv6_count),
        );
        print_barrier();
        print_kv(
            "TCP packet count",
            &format!("{}", self.statistics.tcp_count),
        );
        print_barrier();
        print_kv(
            "UDP packet count",
            &format!("{}", self.statistics.udp_count),
        );
        print_barrier();
        print_kv(
            "ICMP packet count",
            &format!("{}", self.statistics.icmp_count),
        );
        print_barrier();
        print_kv(
            "ICMP redirect",
            &format!("{}", self.statistics.icmp_redirect),
        );
        print_barrier();
        print_kv(
            "ICMP destination unreachable",
            &format!("{}", self.statistics.icmp_dest_unreachable),
        );
        print_barrier();
        print_kv(
            "ICMPv6 packet count",
            &format!("{}", self.statistics.icmpv6_count),
        );
        print_barrier();
        print_kv(
            "ICMPv6 redirect",
            &format!("{}", self.statistics.icmpv6_redirect),
        );
        print_barrier();
        print_kv(
            "ICMPv6 destination unreachable",
            &format!("{}", self.statistics.icmpv6_dest_unreachable),
        );
        print_barrier();
        println!("found MAC addresses: {:?}", self.known_layer2_hosts);
    }
}

fn warn(msg: &str) {
    eprintln!(
        "{} [WARN ] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        msg
    );
}
fn info(msg: &str) {
    println!(
        "{} [INFO ] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        msg
    )
}
fn error(msg: &str) {
    eprintln!(
        "{} [ERROR] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        msg
    );
}

fn main() {
    use pnet::datalink::Channel;

    // 获得命令行参数
    let opt = Opt::from_args();

    // 获得本机所有的网络接口
    let interface = datalink::interfaces()
        .iter()
        .filter(|s| s.name == opt.ifname)
        .next()
        .expect(&format!("Unknown interface {}", opt.ifname))
        .to_owned();

    // 监听某个网络接口的链路层数据
    // 默认启用了混杂模式
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => unreachable!(),
        Err(e) => panic!("Error in creating channel: {}", e),
    };

    // 控制是否分析哪些包
    let mut filter = PacketFilter {
        ..Default::default()
    };
    filter.eth = opt.ethernet;
    filter.payload = opt.hexdump;
    if opt.all {
        filter.arp = true;
        filter.eth = true;
        filter.icmp = true;
        filter.icmpv6 = true;
        filter.ipv4 = true;
        filter.ipv6 = true;
        filter.other = true;
        filter.tcp = true;
        filter.udp = true;
    } else {
        if let Some(protocols) = opt.protocol {
            for protocol in protocols {
                match protocol.as_str() {
                    "arp" => filter.arp = true,
                    "ip" => filter.ipv4 = true,
                    "ipv6" => filter.ipv6 = true,
                    "icmp" => filter.icmp = true,
                    "icmpv6" => filter.icmpv6 = true,
                    "tcp" => filter.tcp = true,
                    "udp" => filter.udp = true,
                    "other" => filter.other = true,
                    _ => (),
                }
            }
        }
    }

    let mut parser = Parser::new(filter);

    // 实现按crtl-c之后输出统计结果
    let (crtlc_tx, crtlc_rx) = mpsc::channel();
    ctrlc::set_handler(move || crtlc_tx.send(()).unwrap()).unwrap();

    // 无限循环抓包
    loop {
        match rx.next() {
            Ok(raw) => {
                // 分析原始数据
                parser.parse(&raw);
                info("packet end")
            }
            Err(err) => {
                if err.kind() == std::io::ErrorKind::Interrupted {
                    println!();
                    // 输出统计结果
                    parser.print_summary();
                    return;
                } else {
                    error(&err.to_string());
                }
            }
        };
        if crtlc_rx.try_recv().is_ok() {
            println!();
            // 输出统计结果
            parser.print_summary();
            return;
        }
    }
}
