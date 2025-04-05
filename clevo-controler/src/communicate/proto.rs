use crate::communicate::CommuError;
use serde::{Deserialize, Serialize};

use super::stream::SocketStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MsgMode {
    Request,
    Reply,
    Notify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MsgError {
    None,                         // No error
    UnsupportedOperation(String), // Unsupported operation
    InvalidParameter(String),     // Invalid parameter
    DeviceError(String),          // Device error
    Timeout(String),              // Timeout error
    Unknown(String),              // Unknown error
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MsgCommand {
    GetCpuInfo,
    GetGpuInfo,
    GetCpuStatus,
    GetGpuStatus,
    SetFreq,
    SetFanSpeed,
    SetPower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgHeader {
    pub version: u8,
    pub timestamp: u64,
    pub length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgPacket {
    pub mode: MsgMode,
    pub error: Option<MsgError>,
    pub sequence: u64, // Unique sequence number for the message
    pub command: MsgCommand,
}

// Msg = MsgHeader + MsgPacket + Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Msg {
    pub header: MsgHeader,
    pub packet: MsgPacket,
    pub payload: Option<String>, // Optional payload, can be empty
}

type Result<T> = std::result::Result<T, CommuError>;

pub fn recv_msg(stream: &mut SocketStream) -> Result<Msg> {
    let msg_header_str = stream.read(size_of::<MsgHeader>())?;
    let msg_header: MsgHeader = serde_json::from_str(&msg_header_str).unwrap(); // Should not failed
    dbg!(msg_header.clone());
    let msg_packet_str = stream.read(size_of::<MsgPacket>())?;
    let msg_packet: MsgPacket = serde_json::from_str(&msg_packet_str).unwrap(); // Should not failed
    let payload_length = msg_header.length - size_of::<MsgHeader>() - size_of::<MsgPacket>();
    let payload = if payload_length > 0 {
        Some(stream.read(payload_length)?)
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
    let msg_header = MsgHeader {
        version: 1, // Version of the message protocol
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64, // Current timestamp in seconds
        length: size_of::<MsgPacket>() + if let Some(p) = payload { p.len() } else { 0 }, // Total length of the message
    };
    let msg_header_str = serde_json::to_string(&msg_header).unwrap(); // Should not failed
    let msg_packet_str = serde_json::to_string(msg_packet).unwrap(); // Should not failed
    stream.write(&msg_header_str)?;
    stream.write(&msg_packet_str)?;
    if let Some(payload) = payload {
        stream.write(payload)?;
    }
    Ok(())
}
