use rosc::OscPacket;

pub struct Parameter {
    value: u32,
}

fn handle_packet(packet: OscPacket) {
    match packet {
        OscPacket::Message(msg) => {
            println!("OSC address: {}", msg.addr);
            println!("OSC arguments: {:?}", msg.args);
        }
        OscPacket::Bundle(bundle) => {
            println!("OSC Bundle: {:?}", bundle);
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rosc;

    use rosc::OscPacket;
    use std::env;
    use std::net::{SocketAddrV4, UdpSocket};
    use std::str::FromStr;
    use crate::parameter::handle_packet;

    #[test]
    fn test_color_enum() {
        let args: Vec<String> = env::args().collect();
        let usage = format!("Usage {} IP:PORT", &args[0]);
        // if args.len() < 2 {
        //     println!("{}", usage);
        //     ::std::process::exit(1)
        // }
        let addr = match SocketAddrV4::from_str( "127.0.0.1:3131" /*&args[1]*/) {
            Ok(addr) => addr,
            Err(_) => panic!(usage),
        };
        let sock = UdpSocket::bind(addr).unwrap();
        println!("Listening to {}", addr);

        let mut buf = [0u8; rosc::decoder::MTU];

        loop {
            match sock.recv_from(&mut buf) {
                Ok((size, addr)) => {
                    println!("Received packet with size {} from: {}", size, addr);
                    let packet = rosc::decoder::decode(&buf[..size]).unwrap();
                    handle_packet(packet);
                }
                Err(e) => {
                    println!("Error receiving from socket: {}", e);
                    break;
                }
            }
        }
    }
}
