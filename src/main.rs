#![feature(let_else)]
extern crate packet_builder;
extern crate pnet;

mod gateway;
mod sendrecv;
use std::{net::Ipv4Addr};
use clap::Parser;
use dns_lookup::lookup_host;



#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Local interface
    #[clap(short, long, value_parser)]
    if_name: String,

    /// Destination hostname(domain or ipv4 address, ipv6 is not tested)
    #[clap(short, long, value_parser)]
    hostname: String,

    /// Destination port
    #[clap(short, long, value_parser, default_value_t = 80)]
    port: u16,
    
    /// Number of times to ping
    #[clap(short, long, value_parser, default_value_t = 10)]
    count: usize
}

fn main() {
    let args = Args::parse();
    let dest_ip = if let Ok(ip) = args.hostname.parse::<Ipv4Addr>() {
        ip
    } else {
        let ips: Vec<std::net::IpAddr> = lookup_host(&args.hostname).unwrap();
        let ip = ips
            .into_iter()
            .filter(|ip| ip.is_ipv4())
            .next()
            .unwrap_or_else(|| panic!("No ipv4 address found for hostname: {}", args.hostname));
        let std::net::IpAddr::V4(ipv4) = ip else {
            unreachable!();
        };
        ipv4
    };
    
    let res = sendrecv::test_latency(&args.if_name, dest_ip, args.port, args.count);
    println!("Valid Result Count: {}", res.len());
    println!("RTTs(us): {}", res.iter().map(|v| (*v as f64 / 1000.).to_string()).collect::<Vec<_>>().join(", "));
    println!("AVG RTT(us): {}", res.iter().sum::<u128>() as f64 / (res.len() as f64 * 1000.));
}
