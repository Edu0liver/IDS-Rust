use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::Packet;
use crate::protocols::{ipv4, ipv6};

pub fn handle(frame: &[u8]) {
    if let Some(eth) = EthernetPacket::new(frame) {
        match eth.get_ethertype() {
            EtherTypes::Ipv4 => ipv4::handle(eth.payload()),
            EtherTypes::Ipv6 => ipv6::handle(eth.payload()),
            _ => {}
        }
    }
}
