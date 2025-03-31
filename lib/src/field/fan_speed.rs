use crate::field::FieldError;
use bincode::{Decode, Encode};

type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct FanSpeed {
    duty: u32, // Fan duty cycle in percentage
    rpm: u32,  //Fan rpm
}

impl FanSpeed {
    pub fn new(duty: u32, rpm: u32) -> Self {
        Self { duty, rpm }
    }
    pub fn get_duty(&self) -> u32 {
        self.duty
    }
    pub fn get_rpm(&self) -> u32 {
        self.rpm
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
}
