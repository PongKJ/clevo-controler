pub mod cpu;
pub mod field;
pub mod gpu;

use field::{
    fan_speed::{FanSpeed, TargetFanSpeed},
    freq::Freq,
    power::Power,
    temp::Temp,
    usage::Usage,
};

use crate::communicate::query::QueryError;
use crate::communicate::stream::StreamClient;
use crate::domain::field::FieldError;

#[derive(Debug)]
pub enum HardwareError {
    FieldError(String),  // Invalid field
    QueryError(String),  // Error during querying hardware
    OperationNotSupport, // Operation not supported by the hardware
}

impl From<FieldError> for HardwareError {
    fn from(err: FieldError) -> Self {
        HardwareError::FieldError(String::from(err))
    }
}
impl From<QueryError> for HardwareError {
    fn from(err: QueryError) -> Self {
        HardwareError::QueryError(String::from(err))
    }
}

type Result<T> = std::result::Result<T, HardwareError>;

// TODO: Split these methods by the catogory of hardware, e.g., CPU/GPU, Disk, NetCard, etc.
pub trait Hardware {
    // Refresh the hardware status
    fn refresh(&mut self, stream_slient: &mut StreamClient) -> Result<()>; // Return error message if failed
    // Get
    fn get_desc(&self) -> &str;

    // Default implementation returns an error if not implemented
    fn get_freq(&self) -> Result<&Freq> {
        Err(HardwareError::OperationNotSupport)
    }
    fn get_usage(&self) -> Result<&Usage> {
        Err(HardwareError::OperationNotSupport)
    }
    fn get_temp(&self) -> Result<&Temp> {
        Err(HardwareError::OperationNotSupport)
    }
    fn get_power(&self) -> Result<&Power> {
        Err(HardwareError::OperationNotSupport)
    }
    fn get_fan_speed(&self) -> Result<&FanSpeed> {
        Err(HardwareError::OperationNotSupport)
    }
    fn set_fan_speed(&self, stream_slient: &mut StreamClient, speed: TargetFanSpeed) -> Result<()> {
        Err(HardwareError::OperationNotSupport)
    }
    fn set_freq(
        &self,
        stream_slient: &mut StreamClient,
        min_freq: u32,
        max_freq: u32,
    ) -> Result<()> {
        Err(HardwareError::OperationNotSupport)
    }
}
