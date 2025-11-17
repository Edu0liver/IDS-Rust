use std::str;

pub fn handle(payload: &[u8], src: &str, dst: &str, src_port: u16, dst_port: u16) {
    if payload.is_empty() {
        return;
    }

    let is_http_port = src_port == 80 || dst_port == 80 ||
        src_port == 8080 || dst_port == 8080 ||
        src_port == 8000 || dst_port == 8000;

    if !is_http_port {
        return;
    }

    if let Ok(text) = str::from_utf8(payload) {
        if text.starts_with("GET") || text.starts_with("POST") || text.contains("HTTP/1.1") {
            println!("===== HTTP DETECTED =====");
            println!("{}:{} -> {}:{}", src, src_port, dst, dst_port);
            println!("--- Requisition ---");
            for line in text.lines().take(8) {
                println!("{}", line);
            }
            println!("===========================");
        }
    }
}
