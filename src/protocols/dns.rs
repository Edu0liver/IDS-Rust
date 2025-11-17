use dns_parser::Packet;

pub fn handle(payload: &[u8], src: &str, dst: &str) {
    if let Ok(packet) = Packet::parse(payload) {
        println!("===== DNS DETECTED =====");
        println!("{} -> {}", src, dst);

        for q in packet.questions {
            println!("DNS Query: {}  ({:?})", q.qname, q.qtype);
        }

        println!("=========================");
    }
}
