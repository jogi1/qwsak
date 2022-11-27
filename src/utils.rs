use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

use quakeworld::utils::ascii_converter::AsciiConverter;

pub fn file_read_full(filename: PathBuf) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    match file.read_to_end(&mut buffer) {
        Ok(size) => size,
        Err(err) => {
            return Err(Box::new(err));
        }
    };
    Ok(buffer)
}


pub fn ascii_table_stip(strip_new_line: bool, buffer: impl Into<Vec<u8>>) -> Vec<u8> {
    let mut buffer = buffer.into();
    if strip_new_line {
        buffer[10] = b'_';
    }
    buffer
}

pub fn ascii_table_new(strip_new_line: bool) -> Vec<u8> {
         let table: &str = "__________\n_____[]0123456789____ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_'abcdefghijklmnopqrstuvwxyz{|}~_________________[]0123456789____ !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_'abcdefghijklmnopqrstuvwxyz{|}~_";
         let mut ascii_table = table.as_bytes().to_vec();
         ascii_table_stip(strip_new_line, ascii_table)
}

pub fn ascii_table_from_file(filename: Option<PathBuf>, strip: bool) -> Result<AsciiConverter, Box<dyn std::error::Error>> {
    let ascii_table: Vec<u8>;
    if let Some(filename) = filename {
        let contents = file_read_full(filename)?;
        ascii_table = ascii_table_stip(strip, contents);
    } else {
        ascii_table = ascii_table_new(strip);
    }
    AsciiConverter::new_with_table(ascii_table)
}

