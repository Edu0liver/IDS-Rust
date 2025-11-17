use pnet::datalink::{self, Channel::Ethernet};

fn main() {
    let interfaces = datalink::interfaces();
    let iface = interfaces
        .into_iter()
        .find(|i| i.is_up() && !i.is_loopback() && i.ips.len() > 0)
        .expect("Nenhuma interface válida encontrada");

    println!("Escutando na interface: {}", iface.name);

    let mut rx = match datalink::channel(&iface, Default::default()) {
        Ok(Ethernet(_, rx)) => rx,
        Ok(_) => panic!("Canal não é Ethernet"),
        Err(e) => panic!("Erro ao abrir canal: {}", e),
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                println!("Pacote capturado: {} bytes", packet.len());
            }
            Err(e) => {
                println!("Erro ao ler pacote: {}", e);
            }
        }
    }
}
