use crate::domain::hardware::field::FieldError;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Default, Serialize, Deserialize)]
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
