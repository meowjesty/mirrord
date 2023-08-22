use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

fn parse_args() -> Option<SocketAddr> {
    let mut args = std::env::args();

    match args.next()?.as_str() {
        "--ip" => args.next().map(|addr| addr.parse::<SocketAddr>().unwrap()),
        "--hostname" => args.next().and_then(|hostname| {
            hostname
                .to_socket_addrs()
                .unwrap()
                .find(SocketAddr::is_ipv4)
        }),
        invalid => panic!("unexpected address_type was passed to this program {invalid}!"),
    }
}

fn main() {
    println!("test outgoing_filter: START");

    let remote_addr = "service.name"
        .to_socket_addrs()
        .expect("Failed converting service.name into addresses!")
        .find(SocketAddr::is_ipv4)
        .expect("No ipv4 addresses found!");
    let _ = TcpStream::connect(remote_addr).unwrap();

    println!("test outgoing_filter: SUCCESS");
}
