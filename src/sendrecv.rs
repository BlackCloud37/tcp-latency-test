use pnet::datalink::{self, NetworkInterface, DataLinkSender, DataLinkReceiver};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use packet_builder::payload::PayloadData;
use packet_builder::*;
use pnet::packet::tcp::TcpFlags;
use pnet::packet::Packet;
use pnet::util::MacAddr;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::tcp::TcpPacket;
use portpicker::pick_unused_port;
use std::net::{IpAddr, Ipv4Addr};
use std::thread::sleep;
use std::time::Instant;
use crate::gateway;


fn get_interface(if_name: &str) -> NetworkInterface {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == if_name)
        .next()
        .unwrap_or_else(|| panic!("No such network interface: {}", if_name));
    interface
}

fn get_sender_receiver(if_name: &str) -> (Box<dyn DataLinkSender>, Box<dyn DataLinkReceiver>) {
    let interface = get_interface(if_name);
    match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    }
}

fn get_gateway_mac_addr(if_name: &str) -> MacAddr {
    let gateway_mac_octets = gateway::get_gateway_mac_octets(if_name);
    let [m1, m2, m3, m4, m5, m6] = gateway_mac_octets;
    MacAddr(m1, m2, m3, m4, m5, m6)
}

fn get_local_ipv4_addr(interface: &NetworkInterface) -> Ipv4Addr {
    let ip = interface.ips
        .iter()
        .filter(|ip| ip.is_ipv4())
        .next()
        .unwrap_or_else(|| panic!("No ipv4 address associated with interface: {}", interface.name))
        .ip();
    let IpAddr::V4(ipv4) = ip else {
        unreachable!();
    };
    ipv4
}

const TIMEOUT: u128 = 3;
pub fn test_latency(if_name: &str, dest_ipv4_addr: std::net::Ipv4Addr, dest_port: u16, iter: usize) -> Vec<u128> {
    let interface = get_interface(if_name);
    let local_ipv4_addr = get_local_ipv4_addr(&interface);
    let gateway_mac = get_gateway_mac_addr(if_name);

    let mut rtts = Vec::with_capacity(iter);

    let (mut sender, mut receiver) = get_sender_receiver(if_name);    
    for i in 0..iter {
        let local_port = pick_unused_port().unwrap_or_else(|| panic!("No local port available."));

        let mut pkt_buf = [0u8; 1500];
        let packet = packet_builder!(
            pkt_buf,
            ether({set_destination => gateway_mac, set_source => interface.mac.unwrap()}) /
            ipv4({set_source => local_ipv4_addr, set_destination => dest_ipv4_addr }) /
            tcp({set_source => local_port, set_destination => dest_port, set_flags => (TcpFlags::SYN)}) /
            payload({[0; 0]})
        );

        let start = Instant::now();
        sender.send_to(packet.packet(), None).unwrap().unwrap();
        let send_duration = start.elapsed().as_nanos();
        loop {
            match receiver.next() {
                Ok(packet) => {
                    let elapsed = start.elapsed().as_nanos();
                    if elapsed > 1000_000_000 * TIMEOUT {
                        println!("Packet({}) receive timeout({}s)", i, TIMEOUT);
                        break;
                    }
                    let packet = EthernetPacket::new(packet).unwrap();
                    if packet.get_ethertype() != EtherTypes::Ipv4 {
                        continue;
                    }
                    let Some(ipv4_pkt) = Ipv4Packet::new(packet.payload()) else {
                        continue;
                    };
                    if ipv4_pkt.get_source() != dest_ipv4_addr || 
                        ipv4_pkt.get_next_level_protocol() != IpNextHeaderProtocols::Tcp {
                        continue;
                    }
                    let Some(tcp_pkt) = TcpPacket::new(ipv4_pkt.payload()) else {
                        continue;
                    };
                    if tcp_pkt.get_source() == dest_port && tcp_pkt.get_destination() == local_port {
                        assert!((tcp_pkt.get_flags() & TcpFlags::SYN) != 0 && (tcp_pkt.get_flags() & TcpFlags::ACK) != 0);
                        let rtt = (elapsed - send_duration / 2) / 2;
                        println!("SYN&ACK({}) from {} time={} us", i, dest_ipv4_addr, rtt / 1000);
                        rtts.push(rtt);
                        break;
                    }
                }
                Err(e) => {
                    panic!("An error occurred while reading: {}", e);
                }
            }
        }
        sleep(std::time::Duration::from_millis(500));
    }
    return rtts;
}