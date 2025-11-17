// #[warn(unused_attributes)]

mod protocols;

use pnet::datalink::{self, Channel::Ethernet};
use protocols::ethernet;

fn main() {
    let interfaces = datalink::interfaces();
    let iface = interfaces
        .into_iter()
        .find(|i| i.is_up() && !i.is_loopback() && i.ips.len() > 0)
        .expect("No valid interface!");

    println!("Listening interface: {}", iface.name);

    let (_, mut rx) = match datalink::channel(&iface, Default::default()) {
        Ok(Ethernet(_tx, rx)) => (_tx, rx),
        Ok(_) => panic!("Channel not Ethernet"),
        Err(e) => panic!("Open channel error: {}", e),
    };

    loop {
        if let Ok(frame) = rx.next() {
            ethernet::handle(frame);
        }
    }
}
