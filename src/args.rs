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
    /// print trace output
    #[clap(subcommand)]
    Trace(TraceCommandType),
    /// sanitize strings
    Sanitize(SanitizeCommand),
    /// parse setinfo strings
    ParseSetinfo(ParseSetinfoCommand),
    /// sends an out of band command
    OobCommand(OobCommandCommand),
}

#[derive(Debug, Subcommand)]
pub enum TraceCommandType {
    /// trace as if message recieved from server
    Connection(TraceCommandConnection),
    /// trace as if message recieved from server, without header
    Raw(TraceCommandRaw),
    /// trace as if message is an mvd
    Mvd(TraceCommandMvd),
}

#[derive(Debug, Args)]
pub struct TraceCommandConnection {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    /// output as json
    pub json: bool,

    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    /// do not colorize output
    pub no_colors: bool,

    #[arg(long, default_value="9  10  11  12  13  14")]
    /// a string of " " separat color codes.
    pub colors: String,

    #[arg(long, default_value="0")]
    /// trace output start depth
    pub trace_depth_start: i32,

    #[arg(long, default_value="-1")]
    /// trace output stop depth, -1 means no limit
    pub trace_depth_stop: i32,


    /// file to trace
    pub file: PathBuf,

    /// flags for parsing
    pub flags: Vec<TraceFlagsProtocol>,
}

#[derive(Debug, Args)]
pub struct TraceCommandRaw {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    /// output as json
    pub json: bool,

    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    /// do not colorize output
    pub no_colors: bool,

    #[arg(long, default_value="9  10  11  12  13  14")]
    /// a string of " " separat color codes.
    pub colors: String,

    #[arg(long, default_value="0")]
    /// trace output start depth
    pub trace_depth_start: i32,

    #[arg(long, default_value="-1")]
    /// trace output stop depth, -1 means no limit
    pub trace_depth_stop: i32,


    /// file to trace
    pub file: PathBuf,

    /// flags for parsing
    pub flags: Vec<TraceFlagsProtocol>,
}

#[derive(Debug, Args)]
pub struct TraceCommandMvd {
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    /// output as json
    pub json: bool,

    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    /// do not colorize output
    pub no_colors: bool,

    #[arg(long, default_value="9  10  11  12  13  14")]
    /// a string of " " separat color codes.
    pub colors: String,

    #[arg(long, default_value="0")]
    /// first frame to trace
    pub frame_start: u32,

    #[arg(long)]
    /// last frame to trace
    pub frame_stop: Option<u32>,

    #[arg(long, default_value="0")]
    /// trace output start depth
    pub trace_depth_start: i32,

    #[arg(long, default_value="-1")]
    /// trace output stop depth, -1 means no limit
    pub trace_depth_stop: i32,


    /// file to trace
    pub file: PathBuf,

    /// flags for parsing
    pub flags: Vec<TraceFlagsProtocol>,
}


#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum TraceFlagsProtocol {
	FtexTrans,
	FtexAccurateTimings,
	FtexHlBsp,
	FtexModeldbl,
	FtexEntitydbl,
	FtexEntitydbl2,
	FtexFloatcoords,
	FtexSpawnstatic2,
	FtexPacketentities256,
	FtexChunkedDownloads,
    Fte2Voicechat,
    Mvd1Floatcoords,
    Mvd1Highlagteleport,
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
    /// command to send
    pub command: String,
}

