use std::{default, fmt::Display};

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
    Reply,
    Notify,
}

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
pub enum MsgError {
    UnsupportedOperation(String), // Unsupported operation
    InvalidCommand(String),       // Invalid parameter
    DeviceError(String),          // Device error
    Timeout(String),              // Timeout error
    ServerError(String),          // Server error
    Unknown(String),              // Unknown error
}

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
pub enum MsgCommand {
    GetComponentList, // get current enabled hardwares' index
    GetStatus,

    SetFreq,
    SetFanSpeed,
}

impl Display for MsgCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgCommand::GetComponentList => write!(f, "GetComponentList"),
            MsgCommand::GetStatus => write!(f, "GetCpuStatus"),
            MsgCommand::SetFreq => write!(f, "SetCpuFreq"),
            MsgCommand::SetFanSpeed => write!(f, "SetCpuFanSpeed"),
        }
    }
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

    pub fn update_timestamp(&mut self) {
        self.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MsgPacket {
    pub mode: MsgMode,
    pub error: Option<MsgError>,
    pub sequence: u64, // Unique sequence number for the message
    pub index: u8,     // hardware index
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
    payload: &Option<Vec<u8>>,
) -> Result<()> {
    let payload_length = if let Some(p) = &payload { p.len() } else { 0 };
    let msg_packet_encoded = bincode::encode_to_vec(msg_packet, bincode::config::standard())?;
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
    let msg_header_encoded = bincode::encode_to_vec(
        &msg_header,
        bincode::config::standard().with_fixed_int_encoding(),
    )?;
    stream.write(&msg_header_encoded)?;
    stream.write(&msg_packet_encoded)?;
    if let Some(payload) = payload {
        stream.write(payload.as_slice())?;
    }
    Ok(())
}
