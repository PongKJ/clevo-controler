pub mod category;
pub mod desc;
pub mod fan_speed;
pub mod freq;
pub mod power;
pub mod temp;
pub mod usage;
use bincode::{Decode, Encode};
use desc::Desc;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum FieldError {
    InvalidValue(String), // Invalid value for a field
    ParseError(String),   // Error during parsing
}

impl std::error::Error for FieldError {}

impl Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            FieldError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl From<FieldError> for String {
    fn from(value: FieldError) -> Self {
        match value {
            FieldError::InvalidValue(msg) => format!("Invalid value: {}", msg),
            FieldError::ParseError(msg) => format!("Parse error: {}", msg),
        }
    }
}

impl From<bincode::error::DecodeError> for FieldError {
    fn from(err: bincode::error::DecodeError) -> Self {
        FieldError::ParseError(format!("decode error: {}", err))
    }
}

impl From<bincode::error::EncodeError> for FieldError {
    fn from(err: bincode::error::EncodeError) -> Self {
        FieldError::ParseError(format!("encode error: {}", err))
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct CpuStatus {
    pub freq: freq::Freq,
    pub usage: usage::Usage,
    pub power: power::Power,
    pub temp: temp::Temp,
}

impl CpuStatus {
    pub fn serialize(&self) -> Result<Vec<u8>, FieldError> {
        Ok(bincode::encode_to_vec(self, bincode::config::standard())?)
    }
    pub fn deserialize(data: &[u8]) -> Result<Self, FieldError> {
        Ok(bincode::decode_from_slice(data, bincode::config::standard())?.0)
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct GpuStatus {
    pub freq: freq::Freq,
    pub power: power::Power,
    pub temp: temp::Temp,
    pub usage: usage::Usage,
    pub fan_speed: fan_speed::FanSpeed,
}

impl GpuStatus {
    pub fn serialize(&self) -> Result<Vec<u8>, FieldError> {
        Ok(bincode::encode_to_vec(self, bincode::config::standard())?)
    }
    pub fn deserialize(data: &[u8]) -> Result<Self, FieldError> {
        Ok(bincode::decode_from_slice(data, bincode::config::standard())?.0)
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct SetCpuFreq(pub freq::TargetFreq);

#[derive(Debug, Clone, Encode, Decode)]
pub struct SetFanSpeed(pub fan_speed::TargetFanSpeed);

#[derive(Debug, Clone, Encode, Decode)]
pub struct SetGpuFreq(pub freq::TargetFreq);

#[derive(Debug, Clone, Encode, Decode)]
pub struct SetGpuFanSpeed(pub fan_speed::TargetFanSpeed);

#[derive(Debug, Clone, Encode, Decode)]
pub struct ComponentList(pub HashMap<u8, Desc>);
impl ComponentList {
    pub fn deserialize(data: &[u8]) -> Result<Self, FieldError> {
        Ok(bincode::decode_from_slice(data, bincode::config::standard())?.0)
    }
}
