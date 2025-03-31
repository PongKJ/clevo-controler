use crate::field::FieldError;
use bincode::{Decode, Encode};
type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct Usage {
    value: u32, // CPU usage in percentage
}

impl Usage {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
    pub fn get_value(&self) -> u32 {
        self.value
    }
}

impl TryFrom<&[u8]> for Usage {
    type Error = FieldError;
    fn try_from(value: &[u8]) -> Result<Self> {
        let (value, _) = bincode::decode_from_slice(value, bincode::config::standard())?;
        Ok(value)
    }
}
