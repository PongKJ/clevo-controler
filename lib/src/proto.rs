use std::usize;

use crate::{
    field::FieldError,
    stream::{SocketStream, StreamError},
};
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode)]
pub enum ProtoError {
    Io(String),    // IO error
    Parse(String), // Parsing error
    Other(String), // Other errors
}

impl From<StreamError> for ProtoError {
    fn from(err: StreamError) -> Self {
        match err {
            StreamError::Io(msg) => ProtoError::Io(msg),
            StreamError::Other(msg) => ProtoError::Other(msg),
        }
    }
}

impl From<FieldError> for ProtoError {
    fn from(err: FieldError) -> Self {
        ProtoError::Parse(format!("Field error: {}", err))
    }
}

impl From<bincode::error::DecodeError> for ProtoError {
    fn from(err: bincode::error::DecodeError) -> Self {
        ProtoError::Parse(format!("Decode error: {}", err))
    }
}

impl From<bincode::error::EncodeError> for ProtoError {
    fn from(err: bincode::error::EncodeError) -> Self {
        ProtoError::Parse(format!("Encode error: {}", err))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Decode, Encode)]
pub enum MsgMode {
    Request,
    Response,
    Notify,
}

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
pub enum MsgError {
    UnsupportedOperation(String), // Unsupported operation
    InvalidParameter(String),     // Invalid parameter
    DeviceError(String),          // Device error
    Timeout(String),              // Timeout error
    Unknown(String),              // Unknown error
}

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
pub enum MsgCommand {
    // CPU
    GetCpuDesc,
    GetCpuFreq,
    GetCpuTemp,
    GetCpuUsage,
    GetCpuPower,
    GetCpuFanSpeed,

    SetCpuFreq,
    SetCpuPower,
    SetCpuFanSpeed,

    // GPU
    GetGpuDesc,
    GetGpuFreq,
    GetGpuTemp,
    GetGpuUsage,
    GetGpuPower,
    GetGpuFanSpeed,

    SetGpuFreq,
    SetGpuFanSpeed,
}

struct Test {
    value: u8,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MsgHeader {
    pub version: u8,
    pub packet_length: u8,
    pub timestamp: u64,
    pub payload_length: usize,
}

// NOTE: std::mem::size_of considers the allignment padding, can't use it
impl MsgHeader {
    pub fn total_field_size() -> usize {
        std::mem::size_of::<u8>()    // Size of `version`
        + std::mem::size_of::<u64>() // Size of `timestamp`
        + std::mem::size_of::<u8>() // Size of `length`
        + std::mem::size_of::<usize>() // Size of `some_other_field`
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MsgPacket {
    pub mode: MsgMode,
    pub error: Option<MsgError>,
    pub sequence: u64, // Unique sequence number for the message
    pub command: MsgCommand,
}

// Msg = MsgHeader + MsgPacket + Payload
#[derive(Debug, Clone, Encode, Decode)]
pub struct Msg {
    pub header: MsgHeader,
    pub packet: MsgPacket,
    pub payload: Option<Vec<u8>>, // Optional payload, can be empty
}

type Result<T> = std::result::Result<T, ProtoError>;

pub fn recv_msg(stream: &mut SocketStream) -> Result<Msg> {
    let msg_header_str = stream.read(MsgHeader::total_field_size())?;
    let (msg_header, _): (MsgHeader, _) = bincode::decode_from_slice(
        msg_header_str.as_slice(),
        bincode::config::standard().with_fixed_int_encoding(),
    )?;
    dbg!(&msg_header);
    let msg_packet_encoded = stream.read(msg_header.packet_length as usize)?;
    dbg!(&msg_packet_encoded);
    let (msg_packet, _): (MsgPacket, _) =
        bincode::decode_from_slice(msg_packet_encoded.as_slice(), bincode::config::standard())?;
    let payload = if msg_header.payload_length > 0 {
        Some(stream.read(msg_header.payload_length)?)
    } else {
        None
    };
    Ok(Msg {
        header: msg_header,
        packet: msg_packet,
        payload, // Can be None if no payload
    })
}

pub fn send_msg(
    stream: &mut SocketStream,
    msg_packet: &MsgPacket,
    payload: &Option<String>,
) -> Result<()> {
    let payload_encoded: Option<Vec<u8>> = if let Some(p) = payload {
        Some(bincode::encode_to_vec(p, bincode::config::standard()).unwrap())
    } else {
        None
    };
    let payload_length = if let Some(p) = &payload_encoded {
        p.len()
    } else {
        0
    };
    let msg_packet_encoded = bincode::encode_to_vec(msg_packet, bincode::config::standard())?;
    dbg!(&msg_packet_encoded);
    let msg_header = MsgHeader {
        version: 1, // Version of the message protocol
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs(), // Current timestamp in seconds
        packet_length: msg_packet_encoded.len() as u8,
        payload_length,
    };
    dbg!(&msg_header);
    let msg_header_str = bincode::encode_to_vec(
        &msg_header,
        bincode::config::standard().with_fixed_int_encoding(),
    )?;
    dbg!(&msg_header_str);
    stream.write(&msg_header_str)?;
    stream.write(&msg_packet_encoded)?;
    if let Some(payload_encoded) = payload_encoded {
        stream.write(payload_encoded.as_slice())?;
    }
    Ok(())
}
