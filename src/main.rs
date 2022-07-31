#![feature(let_else)]
extern crate packet_builder;
extern crate pnet;

mod gateway;
mod sendrecv;
use std::{env, net::Ipv4Addr};
fn main() {
    let help = "Usage: ./tcplatency <interface name> <dest ipv4 addr> <dest port> [count]";
    let if_name = env::args()
        .nth(1)
        .expect(help);
    let dest_ip = env::args()
        .nth(2)
        .expect(help);
    dest_ip.parse::<Ipv4Addr>().expect(&format!("Err: {} is not a valid ipv4 addr", dest_ip));
    let dest_port = env::args()
        .nth(3)
        .expect(help)
        .parse()
        .expect("Err: invalid port");
    let iter = env::args()
        .nth(4)
        .unwrap_or("5".to_string())
        .parse()
        .expect("Err: invalid count");
    let res = sendrecv::test_latency(&if_name, &dest_ip, dest_port, iter);
    println!("Valid Result Count: {}", res.len());
    println!("RTTs(ns): {}", res.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","));
    println!("AVG RTT(ns): {}", res.iter().sum::<u128>() as f64 / res.len() as f64);
}
