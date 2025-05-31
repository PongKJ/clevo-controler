pub mod cpu;
pub mod fan;
pub mod gpu;

use cpu::CpuError;
use lib::field::{FieldError, desc::Desc};
use lib::proto::{MsgCommand, MsgError};
use lib::stream::StreamError;

#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("lowerlevel error: {0}")]
    LowerlevelError(String), // Error during execution
    #[error("field error: {0}")]
    FieldError(String), // Invalid field
    #[error("query error: {0}")]
    QueryError(#[from] StreamError), // Error during querying hardware
    #[error("operation not supported by the hardware")]
    OperationNotSupport, // Operation not supported by the hardware
    #[error("bad reply from daemon")]
    BadReply,
}

impl From<CpuError> for crate::component::ComponentError {
    fn from(err: CpuError) -> Self {
        crate::component::ComponentError::LowerlevelError(err.to_string())
    }
}

impl From<FieldError> for ComponentError {
    fn from(err: FieldError) -> Self {
        ComponentError::FieldError(String::from(err))
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
