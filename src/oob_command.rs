use quakeworld::protocol::message::Message;
use quakeworld::protocol::types;
use std::time::Duration;

use crate::args::OobCommandCommand;
use crate::utils::ascii_table_from_file;

pub fn oob_command (options: OobCommandCommand) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = vec![0u8; 1024 * 4];

    let converter = ascii_table_from_file(options.file, options.strip)?;

    let mut message = Message::empty();
    let socket = crate::network::bind_socket(false, options.local_ip)?;
    socket.connect(options.remote_ip)?;
    socket.set_read_timeout(Some(Duration::new(1, 0)))?;

    message.write_u32(types::OOB_HEADER);
    message.write_stringbyte(options.command);

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
