use std::path::PathBuf;
use clap::{arg, value_parser, Command, Arg, ArgAction};
use std::process::exit;
use std::fs::File;
use std::io::Read;

pub mod network;
pub mod sanatize;
pub mod oob_command;
pub mod parse_setinfo;

#[derive(Default)]
pub struct QwSAKConfig {
    ascii_table: Option<Vec<u8>>,
    debug: bool,
    as_json: bool,
}

fn main() {
    let mut qwsak_cfg = QwSAKConfig::default();
    let matches = Command::new("qwsak")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::new("json")
            .short('j')
            .long("json")
            .action(ArgAction::SetTrue),
            )
        .arg(
            Arg::new("debug")
            .short('d')
            .long("debug")
            .action(ArgAction::SetTrue),
            )
        .arg(
            arg!(
                -a --ascii_table <FILE> "file to replace the builtin ascii table"
                )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
            )
        .subcommand(
            Command::new("sanatize")
                .about("sanatizes strings")
        )
        .subcommand(
            Command::new("parse_setinfo")
                .about("parses setinfo strings")
        )
        .subcommand(
            Command::new("oob_command")
                .about("sends an out of band command to the server")
                .arg(arg!(<remote_ip>))
                .arg(arg!(<command>))
                .arg(arg!(-l --local_ip <LOCAL_IP> "local ip[:port] to bind to"))
        )
        .get_matches();

    if let Some(ascii_table_path) = matches.get_one::<PathBuf>("ascii_table") {
        let mut buffer = Vec::new();
        let mut file = match File::open(ascii_table_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("{} -- ascii_table: {}", err, ascii_table_path.display());
                exit(2);
            }
        };
        match file.read_to_end(&mut buffer) {
            Ok(size) => size,
            Err(err) => {
                eprintln!("{} -- ascii_table: {}", err, ascii_table_path.display());
                exit(2);
            }
        };
        qwsak_cfg.ascii_table = Some(buffer);
    }

    qwsak_cfg.as_json = *matches.get_one::<bool>("json").unwrap();
    qwsak_cfg.debug = *matches.get_one::<bool>("debug").unwrap();

    let mut ret: Result<(), Box<dyn std::error::Error>>;
    ret = Ok(());

    if let Some(_) = matches.subcommand_matches("sanatize") {
        ret = sanatize::sanatize(&qwsak_cfg);
    }

    if let Some(_) = matches.subcommand_matches("parse_setinfo") {
        ret = parse_setinfo::parse(&qwsak_cfg);
    }

    if let Some(matches) = matches.subcommand_matches("oob_command") {
        let local_ip = matches.get_one::<String>("local_ip");
        let remote_ip = matches.get_one::<String>("remote_ip").expect("`remote_ip` is required");
        let command = matches.get_one::<String>("command").expect("`command` is required");
        ret = oob_command::oob_command(&qwsak_cfg, local_ip, remote_ip.to_string(), command.to_string()); 
    }

    match ret {
        Ok(_) => exit(0),
        Err(err) => {
            eprintln!("{}", err);
            exit(2);
        }
    };
}
