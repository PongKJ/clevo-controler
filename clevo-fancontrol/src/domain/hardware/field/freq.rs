use crate::domain::hardware::field::FieldError;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Freq {
    value: u32, // Frequency in MHz
}

impl Freq {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
    pub fn get_value(&self) -> u32 {
        self.value
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
}
