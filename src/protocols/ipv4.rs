use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use pnet::packet::ip::IpNextHeaderProtocols;
use std::net::IpAddr;
use crate::protocols::{tcp, udp};

pub fn handle(payload: &[u8]) {
    if let Some(ip) = Ipv4Packet::new(payload) {
        let src = IpAddr::V4(ip.get_source());
        let dst = IpAddr::V4(ip.get_destination());

        match ip.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => tcp::handle(ip.payload(), src, dst),
            IpNextHeaderProtocols::Udp => udp::handle(ip.payload(), src, dst),
            _ => {}
        }
    }
}
