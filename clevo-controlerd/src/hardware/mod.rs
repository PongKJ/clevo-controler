pub mod cpu;
pub mod gpu;

use lib::field::FieldError;
use lib::proto::{Msg, MsgError};
use lib::stream::StreamError;

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
impl From<bincode::error::EncodeError> for HardwareError {
    fn from(err: bincode::error::EncodeError) -> Self {
        HardwareError::FieldError(format!("Encode error: {}", err))
    }
}
impl From<bincode::error::DecodeError> for HardwareError {
    fn from(err: bincode::error::DecodeError) -> Self {
        HardwareError::FieldError(format!("Decode error: {}", err))
    }
}

#[allow(unused_variables)]
pub trait Hardware {
    fn get_desc(&self) -> String {
        "Unname hardware".to_string()
    }

    // Refresh the hardware status
    fn refresh_status(&mut self) -> Result<(), HardwareError> {
        // Default implementation does nothing
        Ok(())
    }

    fn handle_request(&mut self, msg: &Msg) -> Result<Option<Vec<u8>>, MsgError> {
        Err(MsgError::UnsupportedOperation(format!(
            "Operation not supported by the hardware:{}",
            msg.packet.command
        )))
    }
}
