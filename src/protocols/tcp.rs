use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use std::net::IpAddr;

pub fn handle(payload: &[u8], src_ip: IpAddr, dst_ip: IpAddr) {
    if let Some(tcp) = TcpPacket::new(payload) {
        let src_port = tcp.get_source();
        let dst_port = tcp.get_destination();
        let data = tcp.payload();

        println!(
            "TCP {}:{} -> {}:{} (payload {} bytes)",
            src_ip, src_port, dst_ip, dst_port, data.len()
        );

        if !data.is_empty() {
            if let Ok(text) = std::str::from_utf8(data) {
                println!("Payload (UTF-8 preview): {}", text);
            }
        }
    }
}
