use clap::{ Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct QwsakArgs {
    #[clap(subcommand)]
    pub command: CommandType,
}

#[derive(Debug, Subcommand)]
pub enum CommandType {
    /// sanitize strings
    Sanitize(SanitizeCommand),
    /// parse setinfo strings
    ParseSetinfo(ParseSetinfoCommand),
    /// sends an out of band command
    OobCommand(OobCommandCommand),
}

// Sanatize
#[derive(Debug, Args)]
pub struct SanitizeCommand {
    #[arg(short, long)]
    /// ascii translation table
    pub file: Option<PathBuf>,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    /// strip new lines
    pub strip: bool,
}

// Sanatize
#[derive(Debug, Args)]
pub struct ParseSetinfoCommand {
    #[arg(short, long)]
    /// ascii translation table
    pub file: Option<PathBuf>,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    /// strip new lines
    pub strip: bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    /// output as json
    pub json: bool,
}

// Oob 
#[derive(Debug, Args)]
pub struct OobCommandCommand {
    /// ip:port to send command to
    pub remote_ip: String,
    #[arg(short, long, default_value = "0.0.0.0:0")]
    /// ip:port to send command to
    pub local_ip: String,
    #[arg(short, long)]
    /// ascii translation table
    pub file: Option<PathBuf>,
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    /// strip new lines
    pub strip: bool,

    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    /// output as json
    pub json: bool,

    #[arg(last = true)]
    /// flags for parsing
    pub command: String,
}

