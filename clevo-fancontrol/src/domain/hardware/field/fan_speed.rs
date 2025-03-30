use crate::domain::hardware::field::FieldError;
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, FieldError>;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
