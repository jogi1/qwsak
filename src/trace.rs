use crate::args::{TraceCommandType, TraceFlagsProtocol, TraceCommandMvd, TraceCommandRaw, TraceCommandConnection};
use quakeworld::mvd::Mvd;
use quakeworld::protocol::message::{MessageFlags, Message, MessageType};
use quakeworld::protocol::types::{FteProtocolExtensions, FteProtocolExtensions2, MvdProtocolExtensions, ServerMessage, ServerClient};
use quakeworld::utils::trace::print_message_trace_with_colors;

use crate::utils::file_read_full;

fn protocol_flags_from_flags(flags: Vec<TraceFlagsProtocol>) -> MessageFlags {
    let mut mflags = MessageFlags::new_empty();
    for f in flags {
        match f {
            TraceFlagsProtocol::FtexTrans => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::TRANS) },
            TraceFlagsProtocol::FtexAccurateTimings => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::ACCURATETIMINGS) },
            TraceFlagsProtocol::FtexHlBsp => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::HLBSP) },
            TraceFlagsProtocol::FtexModeldbl => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::MODELDBL) },
            TraceFlagsProtocol::FtexEntitydbl => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::ENTITYDBL) },
            TraceFlagsProtocol::FtexEntitydbl2 => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::ENTITYDBL2) },
            TraceFlagsProtocol::FtexFloatcoords => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::FLOATCOORDS) },
            TraceFlagsProtocol::FtexSpawnstatic2 => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::SPAWNSTATIC2) },
            TraceFlagsProtocol::FtexPacketentities256 => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::PACKETENTITIES_256) },
            TraceFlagsProtocol::FtexChunkedDownloads => {
                mflags.fte_protocol_extensions.insert(FteProtocolExtensions::CHUNKEDDOWNLOADS) },
            TraceFlagsProtocol::Fte2Voicechat => {
                mflags.fte_protocol_extensions_2.insert(FteProtocolExtensions2::FTE_PEXT2_VOICECHAT) },
            TraceFlagsProtocol::Mvd1Floatcoords => {
                mflags.mvd_protocol_extension.insert(MvdProtocolExtensions::FLOATCOORDS) },
            TraceFlagsProtocol::Mvd1Highlagteleport => {
                mflags.mvd_protocol_extension.insert(MvdProtocolExtensions::HIGHLAGTELEPORT) },
        };
    }
    mflags
}

fn color_string_to_colors(color_string: String) -> Vec<u8> {
   color_string.split(' ').map(|s| s.trim())
              .filter(|s| !s.is_empty())
              .map(|s| s.parse().unwrap())
              .collect()  
}

pub fn trace_mvd(options: TraceCommandMvd) -> Result<(), Box<dyn std::error::Error>> {
    let file_content = file_read_full(options.file)?;
    let mut mvd = Mvd::new(file_content, None, true)?;
    let colors = color_string_to_colors(options.colors);

    loop {
        match mvd.parse_frame() {
            Ok(frame) => {
                if let Some(stop) = options.frame_stop {
                    if stop < frame.frame {
                        break;
                    }
                }
                if options.frame_start <= frame.frame {
                    if options.json {
                        let frame_json = serde_json::to_string(&frame)?;
                        println!("{}", frame_json);
                        let tr = serde_json::to_string(&mvd.message.trace)?;
                        println!("{}", tr);
                    } else {
                        println!("{:?}", frame);
                        match print_message_trace_with_colors(&mvd.message, true, options.trace_depth_start, options.trace_depth_stop, options.no_colors, colors.clone()) {
                            Ok(..) => {},
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                    mvd.message.trace.clear();
                }
            },
            Err(e) => {
                println!("{}", e);
                break;
            },

        };
    }
    Ok(())
}

pub fn trace(options: TraceCommandType) -> Result<(), Box<dyn std::error::Error>> {
    match options {
        TraceCommandType::Raw(option) => {
            trace_raw(option)
        },
        TraceCommandType::Connection(option) => {
            trace_connection(option)
        },
        TraceCommandType::Mvd(option) => {
            trace_mvd(option)
        },
    }
}

pub fn trace_raw(options: TraceCommandRaw) -> Result<(), Box<dyn std::error::Error>> {
    let flags = protocol_flags_from_flags(options.flags);
    let file_content = file_read_full(options.file)?;
    let colors = color_string_to_colors(options.colors);

    let size = file_content.len();
    let mut message = Message::new(Box::new(file_content), 0, size, false, flags, None, MessageType::Connection);
    message.trace.enabled = true;

    let mut messages: Vec<ServerMessage> = vec![];
    loop {
        message.trace.annotation = Some("message type".to_string());
        let t = match message.read_u8(false) {
            Ok(t) => t,
            Err(e) => {
                println!("{:#?}", e);
                break;
            },
        };

        let cmd = match ServerClient::try_from(t) {
            Ok(cmd) => cmd,
            Err(e) => {
                println!("{:#?}", e);
                break;
            }
        };
        let ret = cmd.read_message(&mut message)?;
        messages.push(ret);
    }

    if options.json {
        let msgs = serde_json::to_string(&messages)?;
        println!("{}", msgs);
        let tr = serde_json::to_string(&message.trace)?;
        println!("{}", tr);
    } else {
        println!("{:?}", messages);
        match print_message_trace_with_colors(&message, true, options.trace_depth_start, options.trace_depth_stop, options.no_colors, colors) {
            Ok(..) => {},
            Err(e) => {
                return Err(e);
            }
        }
        message.trace.clear();
    }
    Ok(())
}

pub fn trace_connection(options: TraceCommandConnection) -> Result<(), Box<dyn std::error::Error>> {
    let flags = protocol_flags_from_flags(options.flags);
    let file_content = file_read_full(options.file)?;
    let colors = color_string_to_colors(options.colors);

    let size = file_content.len();
    let mut message = Message::new(Box::new(file_content), 0, size, false, flags, None, MessageType::Connection);
    message.trace.enabled = true;

    let p = message.read_packet()?;
    if options.json {
        let packet = serde_json::to_string(&p)?;
        println!("{}", packet);
        let tr = serde_json::to_string(&message.trace)?;
        println!("{}", tr);
    } else {
        println!("{:?}", p);
        match print_message_trace_with_colors(&message, true, options.trace_depth_start, options.trace_depth_stop, options.no_colors, colors) {
            Ok(..) => {},
            Err(e) => {
                return Err(e);
            }
        }
        message.trace.clear();
    }
    Ok(())
}
