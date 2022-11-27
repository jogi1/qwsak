use std::io::{Read, Write};
use std::io;
use crate::args::SanitizeCommand;

use crate::utils::ascii_table_from_file;

pub fn sanitize(options: SanitizeCommand) -> Result<(), Box<dyn std::error::Error>> {
    let input = io::stdin();
    let mut out = io::stdout();
    let converter = ascii_table_from_file(options.file, options.strip)?;

    for i in input.bytes() {
        let c = converter.convert_single(i.unwrap());
        let n = out.write(&[c])?;
        assert_eq!(n, 1);
    }
    Ok(())
}
