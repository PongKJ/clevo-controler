use crate::domain::hardware::field::FieldError;
use serde::{Deserialize, Serialize};
type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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
}
