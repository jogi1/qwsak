use std::error::Error;
use std::net::{UdpSocket, SocketAddr};

pub fn bind_socket(debug: bool, local_ip: String) -> Result<UdpSocket, Box<dyn Error>> {
    let mut s: String = local_ip;
    if !s.contains(':') {
        s.push_str(":0");
    }
    let local_address: SocketAddr = s.parse()?;

    match UdpSocket::bind(local_address.to_string()) {
        Ok(socket) => {
            if debug {
                println!("DBG: bound to {}", socket.local_addr().unwrap());
            }
            Ok(socket)
        },
        Err(err) => {
            Err(Box::new(err))
        },
    }
}
