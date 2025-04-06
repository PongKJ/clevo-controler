use crate::field::FieldError;
use bincode::{Decode, Encode};
type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Decode, Encode)]
pub struct Power {
    value: u64, // Power consumption in mWatts
}

impl Power {
    pub fn new(value: u64) -> Self {
        Self { value }
    }
    pub fn get_value(&self) -> u64 {
        self.value
    }
    pub fn set_value(&mut self, value: u64) {
        self.value = value;
    }
}

impl TryFrom<&[u8]> for Power {
    type Error = FieldError;
    fn try_from(value: &[u8]) -> Result<Self> {
        let (value, _) = bincode::decode_from_slice(value, bincode::config::standard())?;
        Ok(value)
    }
}

#[derive(Debug, Default, Clone, Decode, Encode)]
pub struct TargetPower {
    max: u64, // Power consumption in Watts
}

impl TargetPower {
    pub fn new(value: u64) -> Self {
        Self { max: value }
    }
    pub fn get_max(&self) -> u64 {
        self.max
    }
}
