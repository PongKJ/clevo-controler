use crate::field::FieldError;
use bincode::{Decode, Encode};
type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Decode, Encode)]
pub struct Power {
    value: f32, // Power consumption in Watts
}

impl Power {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
    pub fn get_value(&self) -> f32 {
        self.value
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
    max: f32, // Power consumption in Watts
}

impl TargetPower {
    pub fn new(value: f32) -> Self {
        Self { max: value }
    }
    pub fn get_max(&self) -> f32 {
        self.max
    }
}
