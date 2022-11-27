use std::process::exit;

use args::QwsakArgs;
use clap::Parser;

pub mod network;
pub mod sanitize;
pub mod oob_command;
pub mod parse_setinfo;
pub mod trace;
pub mod args;
pub mod utils;


fn exit_process(ret: Result<(), Box<dyn std::error::Error>>) {
    match ret {
        Ok(_) => exit(0),
        Err(err) => {
            eprintln!("{}", err);
            exit(2);
        }
    };
}

fn main() {
    let args = QwsakArgs::parse();
    let ret = match args.command {
        args::CommandType::Sanitize(options) => {
            sanitize::sanitize(options)
        },
        args::CommandType::ParseSetinfo(options) => {
            parse_setinfo::parse(options)
        },
        args::CommandType::OobCommand(options) => {
            oob_command::oob_command(options)
        },
    };

    exit_process(ret);
}
