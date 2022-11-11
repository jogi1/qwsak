use quakeworld::utils::ascii_converter::AsciiConverter;
use std::io::{Read, Write};
use std::io;
use crate::QwSAKConfig;

pub fn sanatize(qwsak_cfg: &QwSAKConfig) -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut out = io::stdout();
    let converter: AsciiConverter;
    if qwsak_cfg.ascii_table.is_some() {
        let table = qwsak_cfg.ascii_table.as_ref().unwrap();
        converter = AsciiConverter::new_with_table(Box::new(table.to_vec()))?;
    } else {
        converter = AsciiConverter::new();
    }

    for i in input.bytes() {
        let c = converter.convert_single(i.unwrap());
        out.write(&[c])?;
    }
    return Ok(());
}
