use quakeworld::utils::userinfo::Userinfo;
use quakeworld::protocol::types::StringByte;
use quakeworld::utils::ascii_converter::AsciiConverter;
use crate::QwSAKConfig;

use std::io;
use std::io::Read;
use serde_json;

pub fn parse(qwsak_cfg: &QwSAKConfig) -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();

    let converter = AsciiConverter::new_with_table(qwsak_cfg.ascii_table.clone())?;
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
    if qwsak_cfg.as_json {
        let j = serde_json::to_string(&userinfo.values)?;
        println!("{}", j);
    } else {
        for (k, v) in userinfo.values {
            println!("{:?} {:?}", k, v);
        }
    }
    Ok(())
}
