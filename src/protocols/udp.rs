use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::net::IpAddr;
use crate::protocols::dns;

pub fn handle(payload: &[u8], src_ip: IpAddr, dst_ip: IpAddr) {
    if let Some(udp) = UdpPacket::new(payload) {
        let src = udp.get_source();
        let dst = udp.get_destination();
        let data = udp.payload();

        println!(
            "UDP {}:{} -> {}:{} (payload {} bytes)",
            src_ip, src, dst_ip, dst, data.len()
        );

        if src == 53 || dst == 53 {
            dns::handle(data, &src_ip.to_string(), &dst_ip.to_string());
        }
    }
}