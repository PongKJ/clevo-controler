pub mod cpu;
pub mod gpu;

use lib::field::FieldError;
use lib::{
    proto::MsgCommand,
    stream::{SocketStream, StreamError},
};

#[derive(Debug)]
pub enum HardwareError {
    FieldError(String),  // Invalid field
    QueryError(String),  // Error during querying hardware
    OperationNotSupport, // Operation not supported by the hardware
    BadReply,
}

impl From<FieldError> for HardwareError {
    fn from(err: FieldError) -> Self {
        HardwareError::FieldError(String::from(err))
    }
}
impl From<StreamError> for HardwareError {
    fn from(err: StreamError) -> Self {
        HardwareError::QueryError(format!("Stream error: {}", err))
    }
}

type Result<T> = std::result::Result<T, HardwareError>;

#[allow(unused_variables)]
pub trait Hardware {
    // Refresh the hardware status
    fn refresh_status(&mut self, command: &MsgCommand, data: &String) -> Result<()> {
        // Default implementation does nothing
        Ok(())
    }
}
