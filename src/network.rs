use std::error::Error;
use std::net::{UdpSocket, SocketAddr};

pub fn bind_socket(debug: bool, local_ip_in: Option<&String>) -> Result<UdpSocket, Box<dyn Error>> {
    let local_address: SocketAddr = if let Some(local_ip_in) = local_ip_in {
        let mut s: String = local_ip_in.to_string();
        if !s.contains(':') {
            s.push_str(":0");
        }
        s.parse()?
    } else {
        "0.0.0.0:0".parse()?
    };

    match UdpSocket::bind(local_address.to_string()) {
        Ok(socket) => {
            if debug {
                println!("DBG: bound to {}", socket.local_addr().unwrap());
            }
            return Ok(socket);
        },
        Err(err) => {
            return Err(Box::new(err));
        },
    };
}
