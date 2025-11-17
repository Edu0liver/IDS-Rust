use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use std::net::IpAddr;
use crate::protocols::{tcp, udp};

pub fn handle(payload: &[u8]) {
    if let Some(ip) = Ipv6Packet::new(payload) {
        let src = IpAddr::V6(ip.get_source());
        let dst = IpAddr::V6(ip.get_destination());

        match ip.get_next_header() {
            IpNextHeaderProtocols::Tcp => tcp::handle(ip.payload(), src, dst),
            IpNextHeaderProtocols::Udp => udp::handle(ip.payload(), src, dst),
            _ => {}
        }
    }
}
