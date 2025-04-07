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
    version: u8,
    timestamp: u64,
    packet_length: u32,
    payload_length: u32,
}

// NOTE: std::mem::size_of considers the allignment padding, can't use it
impl MsgHeader {
    pub fn total_field_size() -> usize {
        std::mem::size_of::<u8>()    // Size of `version`
        + std::mem::size_of::<u64>() // Size of `timestamp`
        + std::mem::size_of::<u32>() // Size of `length`
        + std::mem::size_of::<u32>() // Size of `some_other_field`
    }

    pub fn new(version: u8, packet_length: usize, payload_length: usize) -> Self {
        Self {
            version,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            packet_length: packet_length as u32,
            payload_length: payload_length as u32,
        }
    }
    pub fn get_version(&self) -> u8 {
        self.version
    }
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
    pub fn get_packet_length(&self) -> u32 {
        self.packet_length
    }
    pub fn get_payload_length(&self) -> u32 {
        self.payload_length
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MsgPacket {
    mode: MsgMode,
    error: Option<MsgError>,
    sequence: u64, // Unique sequence number for the message
    id_num: u8,
    command: MsgCommand,
}

impl MsgPacket {
    pub fn new(
        mode: MsgMode,
        error: Option<MsgError>,
        sequence: u64,
        id_num: u8,
        command: MsgCommand,
    ) -> Self {
        Self {
            mode,
            error,
            sequence,
            id_num,
            command,
        }
    }
    pub fn get_mode(&self) -> &MsgMode {
        &self.mode
    }
    pub fn get_error(&self) -> &Option<MsgError> {
        &self.error
    }
    pub fn get_sequence(&self) -> u64 {
        self.sequence
    }
    pub fn get_command(&self) -> &MsgCommand {
        &self.command
    }
    pub fn get_id_num(&self) -> u8 {
        self.id_num
    }
    pub fn set_id_num(&mut self, id_num: u8) {
        self.id_num = id_num;
    }
    pub fn set_mode(&mut self, mode: MsgMode) {
        self.mode = mode;
    }
    pub fn set_error(&mut self, error: MsgError) {
        self.error = Some(error);
    }
    pub fn serialize(&self) -> Result<Vec<u8>> {
        bincode::encode_to_vec(self, bincode::config::standard()).map_err(|e| ProtoError::from(e))
    }
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        bincode::decode_from_slice(data, bincode::config::standard())
            .map(|(msg, _)| msg)
            .map_err(|e| ProtoError::from(e))
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MsgBody {
    packet: Vec<u8>,
    payload: Option<Vec<u8>>,
}
impl MsgBody {
    pub fn new(packet: Vec<u8>, payload: Option<Vec<u8>>) -> Self {
        Self { packet, payload }
    }
    pub fn get_packet(&self) -> &Vec<u8> {
        &self.packet
    }
    pub fn get_payload(&self) -> &Option<Vec<u8>> {
        &self.payload
    }
}

// Msg = MsgHeader + MsgBody(MsgPacket + payload)
#[derive(Debug, Clone, Encode, Decode)]
pub struct Msg {
    pub header: MsgHeader,
    pub body: MsgBody,
}

type Result<T> = std::result::Result<T, ProtoError>;

pub fn recv_msg(stream: &mut SocketStream) -> Result<MsgBody> {
    let msg_header_bin = stream.read(MsgHeader::total_field_size())?;
    let (msg_header, _): (MsgHeader, _) = bincode::decode_from_slice(
        msg_header_bin.as_slice(),
        bincode::config::standard().with_fixed_int_encoding(),
    )?;
    let msg_packet = stream.read(msg_header.packet_length as usize)?;
    let payload = if msg_header.payload_length > 0 {
        Some(stream.read(msg_header.payload_length as usize)?)
    } else {
        None
    };
    Ok(MsgBody {
        packet: msg_packet,
        payload,
    })
}

pub fn send_msg(stream: &mut SocketStream, body: &MsgBody) -> Result<()> {
    let payload_length = if let Some(p) = &body.payload {
        p.len()
    } else {
        0
    };
    let msg_header = MsgHeader::new(1, body.packet.len(), payload_length);
    let msg_header_bin = bincode::encode_to_vec(
        &msg_header,
        bincode::config::standard().with_fixed_int_encoding(),
    )?;
    stream.write(&msg_header_bin)?;
    stream.write(body.packet.as_slice())?;
    if let Some(payload) = &body.payload {
        stream.write(payload)?;
    }
    Ok(())
}
