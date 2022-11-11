use std::error::Error;
use std::net::{UdpSocket, SocketAddr};
use local_ip_address::local_ip;

pub fn bind_socket(debug: bool, local_ip_in: Option<&String>) -> Result<UdpSocket, Box<dyn Error>> {
    let local_address: SocketAddr;

    if local_ip_in.is_some() {
        let mut s: String = local_ip_in.unwrap().to_owned();
        if !s.contains(":") {
            s.push_str(":0");
        }
        local_address = s.parse()?;
    } else {
        let local_ip_res = local_ip().unwrap();
        local_address = SocketAddr::new(local_ip_res, 0);
    }

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
