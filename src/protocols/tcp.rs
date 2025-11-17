use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use std::net::IpAddr;
use crate::protocols::http;

pub fn handle(payload: &[u8], src_ip: IpAddr, dst_ip: IpAddr) {
    if let Some(tcp) = TcpPacket::new(payload) {
        let src_port = tcp.get_source();
        let dst_port = tcp.get_destination();
        let data = tcp.payload();

        println!(
            "TCP {}:{} -> {}:{} ({} bytes)",
            src_ip, src_port, dst_ip, dst_port, data.len()
        );

        http::handle(
            data,
            &src_ip.to_string(),
            &dst_ip.to_string(),
            src_port,
            dst_port,
        );
    }
}
