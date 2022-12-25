// use std::io::Write;
// use std::net::{TcpStream, SocketAddr, IpAddr, Ipv4Addr};

// pub fn send(message: &[u8]) {
//     let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
//     let mut stream = TcpStream::connect(addr).unwrap();

//     stream.write_all(&message);
// }
