use crate::field::FieldError;
use bincode::{Decode, Encode};

type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct FanSpeed {
    rpm: u32, //Fan rpm
}

impl FanSpeed {
    pub fn new(rpm: u32) -> Self {
        Self { rpm }
    }
    pub fn get_rpm(&self) -> u32 {
        self.rpm
    }
    pub fn set_rpm(&mut self, rpm: u32) {
        self.rpm = rpm;
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        let payload = bincode::encode_to_vec(self, bincode::config::standard())?;
        Ok(payload)
    }

    pub fn deserialize(payload: &[u8]) -> Result<Self> {
        let (value, _) = bincode::decode_from_slice(payload, bincode::config::standard())?;
        Ok(value)
    }
}

impl TryFrom<&[u8]> for FanSpeed {
    type Error = FieldError;
    fn try_from(value: &[u8]) -> Result<Self> {
        let (value, _) = bincode::decode_from_slice(value, bincode::config::standard())?;
        Ok(value)
    }
}

#[derive(Debug, Default, Clone, Encode, Decode)]
pub enum FanIndex {
    #[default]
    All = 0,
    Cpu = 1,
    Gpu = 2,
}

impl FanIndex {
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let payload = bincode::encode_to_vec(self, bincode::config::standard())?;
        Ok(payload)
    }

    pub fn deserialize(payload: &[u8]) -> Result<Self> {
        let (value, _) = bincode::decode_from_slice(payload, bincode::config::standard())?;
        Ok(value)
    }
}

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct TargetFanSpeed {
    duty: u32, // Fan duty cycle in percentage
}

impl TargetFanSpeed {
    pub fn new(duty: u32) -> Self {
        Self { duty }
    }
    pub fn get_duty(&self) -> u32 {
        self.duty
    }
    pub fn serialize(&self) -> Result<Vec<u8>> {
        let payload = bincode::encode_to_vec(self, bincode::config::standard())?;
        Ok(payload)
    }

    pub fn deserialize(payload: &[u8]) -> Result<Self> {
        let (value, _) = bincode::decode_from_slice(payload, bincode::config::standard())?;
        Ok(value)
    }
}
