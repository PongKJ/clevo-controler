pub mod cpu;
pub mod fan;
// pub mod gpu;

use cpu::CpuError;
use lib::field::{FieldError, desc::Desc};
use lib::proto::{MsgCommand, MsgError};
use lib::stream::StreamError;

#[derive(Debug)]
pub enum ComponentError {
    LowerlevelError(String), // Error during execution
    FieldError(String),      // Invalid field
    QueryError(String),      // Error during querying hardware
    OperationNotSupport,     // Operation not supported by the hardware
    BadReply,
}

impl From<CpuError> for crate::component::ComponentError {
    fn from(err: CpuError) -> Self {
        crate::component::ComponentError::LowerlevelError(String::from(err))
    }
}

impl From<FieldError> for ComponentError {
    fn from(err: FieldError) -> Self {
        ComponentError::FieldError(String::from(err))
    }
}
impl From<StreamError> for ComponentError {
    fn from(err: StreamError) -> Self {
        ComponentError::QueryError(format!("Stream error: {}", err))
    }
}
impl From<bincode::error::EncodeError> for ComponentError {
    fn from(err: bincode::error::EncodeError) -> Self {
        ComponentError::FieldError(format!("Encode error: {}", err))
    }
}
impl From<bincode::error::DecodeError> for ComponentError {
    fn from(err: bincode::error::DecodeError) -> Self {
        ComponentError::FieldError(format!("Decode error: {}", err))
    }
}

#[allow(unused_variables)]
pub trait Component {
    fn get_desc(&self) -> Desc;

    // Refresh the hardware status
    fn refresh_status(&mut self) -> Result<(), ComponentError> {
        // Default implementation does nothing
        Ok(())
    }

    fn handle_command(
        &mut self,
        command: &MsgCommand,
        payload: &[Vec<u8>],
    ) -> Result<Vec<Vec<u8>>, MsgError> {
        Err(MsgError::UnsupportedOperation(format!(
            "Operation not supported by the hardware:{}",
            command
        )))
    }
}
