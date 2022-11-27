use std::io;
use std::io::Read;
use serde_json;

use quakeworld::utils::userinfo::Userinfo;
use quakeworld::protocol::types::StringByte;

use crate::args::ParseSetinfoCommand;
use crate::utils::ascii_table_from_file;


pub fn parse(options: ParseSetinfoCommand) -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();

    let converter = ascii_table_from_file(options.file, options.strip)?;
    let mut userinfo = Userinfo::new_with_ascii_converter(converter);

    let mut buf = Vec::new();

    for i in input.bytes() {
        buf.push(i.unwrap());
    }

    let sb = StringByte {
        bytes: buf.clone(),
        string:"".to_string() 
    };
    userinfo.update(&sb);
    if options.json {
        let j = serde_json::to_string(&userinfo.values)?;
        println!("{}", j);
    } else {
        for (k, v) in userinfo.values {
            println!("{:?} {:?}", k, v);
        }
    }
    Ok(())
}
