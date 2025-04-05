use crate::field::FieldError;
use bincode::{Decode, Encode};
type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct Temp {
    value: f32, // Temperature in Celsius
}

impl Temp {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
    pub fn get_value(&self) -> f32 {
        self.value
    }
    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }
}

impl TryFrom<&[u8]> for Temp {
    type Error = FieldError;
    fn try_from(value: &[u8]) -> Result<Self> {
        let (value, _) = bincode::decode_from_slice(value, bincode::config::standard())?;
        Ok(value)
    }
}
