use std::fmt::Display;
use crate::{
    field::FieldError,
    stream::{SocketStream, StreamError},
};
use bincode::{Decode, Encode};

#[derive(Debug, Clone, Encode, Decode, thiserror::Error)]
pub enum ProtoError {
    #[error("IO Error: {0}")]
    Io(String), // IO error
    #[error("Parse Error: {0}")]
    Parse(String), // Parsing error
    #[error("Unsupported Operation: {0}")]
    Other(String), // Other errors
}

impl From<StreamError> for ProtoError {
    fn from(err: StreamError) -> Self {
        match err {
            StreamError::Io(msg) => ProtoError::Io(msg.to_string()),
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

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode, thiserror::Error)]
pub enum MsgError {
    #[error("Field Error: {0}")]
    UnsupportedOperation(String), // Unsupported operation
    #[error("Field Error: {0}")]
    InvalidCommand(String), // Invalid parameter
    #[error("Field Error: {0}")]
    DeviceError(String), // Device error
    #[error("Field Error: {0}")]
    Timeout(String), // Timeout error
    #[error("Field Error: {0}")]
    ServerError(String), // Server error
    #[error("Field Error: {0}")]
    Unknown(String), // Unknown error
}

#[derive(Debug, Clone, PartialEq, Eq, Decode, Encode)]
pub enum MsgCommand {
    // Get
    GetComponentList, // Get current enabled hardwares' index
    GetStatus,
    GetFanSpeed,

    // Set
    SetFreq,
    SetFanSpeed,
    SetFanAuto,
}

impl Display for MsgCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MsgCommand::GetComponentList => write!(f, "GetComponentList"),
            MsgCommand::GetStatus => write!(f, "GetCpuStatus"),
            MsgCommand::SetFreq => write!(f, "SetCpuFreq"),
            MsgCommand::SetFanSpeed => write!(f, "SetCpuFanSpeed"),
            MsgCommand::GetFanSpeed => write!(f, "GetCpuFanSpeed"),
            MsgCommand::SetFanAuto => write!(f, "SetCpuAuto"),
        }
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MsgHeader {
    version: u8,
    timestamp: u64,
    packet_length: u32,
}

// NOTE: std::mem::size_of considers the allignment padding, can't use it
impl MsgHeader {
    const FIELD_SIZE: usize = std::mem::size_of::<u8>()    // Size of `version`
        + std::mem::size_of::<u64>() // Size of `timestamp`
        + std::mem::size_of::<u32>(); // Size of `length`

    pub fn new(version: u8, packet_length: usize) -> Self {
        Self {
            version,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            packet_length: packet_length as u32,
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
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MsgPacket {
    mode: MsgMode,
    error: Option<MsgError>,
    sequence: u64, // Unique sequence number for the message
    id_num: u8,
    command: MsgCommand,
    payload_length: Vec<u32>, // Maybe multiple payloads
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
            payload_length: vec![], // Placeholder, will be set later
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
    pub fn get_payload_length(&self) -> &Vec<u32> {
        &self.payload_length
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
        Ok(bincode::encode_to_vec(self, bincode::config::standard())?)
    }
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        Ok(bincode::decode_from_slice(data, bincode::config::standard())?.0)
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct MsgBody {
    packet: MsgPacket,
    payload: Vec<Vec<u8>>,
}
impl MsgBody {
    pub fn new(packet: MsgPacket, payload: Vec<Vec<u8>>) -> Self {
        let mut packet = packet;
        // Set the payload length
        packet.payload_length = payload.iter().map(|p| p.len() as u32).collect();
        Self { packet, payload }
    }
    pub fn get_packet(&self) -> &MsgPacket {
        &self.packet
    }
    pub fn get_payload(&self) -> &Vec<Vec<u8>> {
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
    let msg_header_bin = stream.read(MsgHeader::FIELD_SIZE)?;
    let (msg_header, _): (MsgHeader, _) = bincode::decode_from_slice(
        msg_header_bin.as_slice(),
        bincode::config::standard().with_fixed_int_encoding(),
    )?;
    let msg_packet =
        MsgPacket::deserialize(stream.read(msg_header.packet_length as usize)?.as_slice())?;
    let payload = if !msg_packet.payload_length.is_empty() {
        let mut payload = Vec::new();
        for length in &msg_packet.payload_length {
            let payload_bin = stream.read(*length as usize)?;
            payload.push(payload_bin);
        }
        payload
    } else {
        vec![]
    };
    Ok(MsgBody {
        packet: msg_packet,
        payload,
    })
}

pub fn send_msg(stream: &mut SocketStream, body: &MsgBody) -> Result<()> {
    let msg_packet_bin = body.packet.serialize()?;
    let msg_header = MsgHeader::new(1, msg_packet_bin.len());
    let msg_header_bin = bincode::encode_to_vec(
        &msg_header,
        bincode::config::standard().with_fixed_int_encoding(),
    )?;
    stream.write(&msg_header_bin)?;
    stream.write(msg_packet_bin.as_slice())?;
    if !body.packet.payload_length.is_empty() {
        for payload in &body.payload {
            stream.write(payload)?;
        }
    }
    Ok(())
}
