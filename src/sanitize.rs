use quakeworld::utils::ascii_converter::AsciiConverter;
use std::io::{Read, Write};
use std::io;
use crate::QwSAKConfig;

pub fn sanitize(qwsak_cfg: &QwSAKConfig) -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut out = io::stdout();
    let converter = AsciiConverter::new_with_table(qwsak_cfg.ascii_table.clone())?;

    for i in input.bytes() {
        let c = converter.convert_single(i.unwrap());
        let n = out.write(&[c])?;
        assert_eq!(n, 1);
    }
    Ok(())
}
