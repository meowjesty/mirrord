use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

fn main() {
    println!("test outgoing_filter: START");

    let remote_addr = "metalbear-hostname:7777"
        .to_socket_addrs()
        .expect("Failed converting service.name into addresses!")
        .find(SocketAddr::is_ipv4)
        .expect("No ipv4 addresses found!");
    let _ = TcpStream::connect(remote_addr).expect("Failed connecting to {remote_addr:#?}!");

    println!("test outgoing_filter: SUCCESS");
}
