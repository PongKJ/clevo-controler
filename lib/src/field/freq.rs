use crate::field::FieldError;
use bincode::{Decode, Encode};

type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct Freq {
    value: Vec<u64>, // Frequency in MHz
}

impl Freq {
    pub fn new(value: Vec<u64>) -> Self {
        Self { value }
    }
    pub fn get_value(&self) -> &Vec<u64> {
        &self.value
    }
    pub fn set_value(&mut self, value: Vec<u64>) {
        self.value = value;
    }
}

impl TryFrom<&[u8]> for Freq {
    type Error = FieldError;
    fn try_from(value: &[u8]) -> Result<Self> {
        let (value, _) = bincode::decode_from_slice(value, bincode::config::standard())?;
        Ok(value)
    }
}

#[derive(Debug, Default, Clone, Encode, Decode)]
pub struct TargetFreq {
    min: u32,
    max: u32,
}

impl TargetFreq {
    pub fn new(min: u32, max: u32) -> Self {
        Self { min, max }
    }
    pub fn get_min(&self) -> u32 {
        self.min
    }
    pub fn get_max(&self) -> u32 {
        self.max
    }

    pub fn serialize(&self) -> Result<Vec<u8>> {
        Ok(bincode::encode_to_vec(self, bincode::config::standard())?)
    }

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        Ok(bincode::decode_from_slice(data, bincode::config::standard())?.0)
    }
}
