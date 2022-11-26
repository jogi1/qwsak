use quakeworld::protocol::message::Message;
use quakeworld::protocol::types;
use quakeworld::utils::ascii_converter::AsciiConverter;
use std::time::Duration;
use crate::QwSAKConfig;

pub fn oob_command (qwsak_cfg: &QwSAKConfig, local_ip: Option<&String>, remote_ip: String, command: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0u8; 1024 * 4];

    let converter = AsciiConverter::new_with_table(qwsak_cfg.ascii_table.clone())?;

    let mut message = Message::empty();
    let socket = crate::network::bind_socket(qwsak_cfg.debug, local_ip)?;
    socket.connect(remote_ip)?;
    socket.set_read_timeout(Some(Duration::new(1, 0)))?;

    message.write_u32(types::OOB_HEADER);
    message.write_stringbyte(command);

    socket.send(&message.buffer)?;
    let mut first = true;
    loop {
        match socket.recv_from(&mut buf) {
            Ok((n, _)) => {
                let _s: Vec<u8>;

                if first {
                    first = false;
                    if n > 5 {
                        _s = buf[5..n].to_vec();
                    } else {
                        _s = buf[0..n].to_vec();
                    }
                } else {
                    _s = buf[0..n].to_vec();
                }
                let s = converter.convert(_s);
                print!("{}", s);
            },
            Err(..) => {
                println!();
                break;
            }
        };
    }
    Ok(())
}
