pub mod cpu;
pub mod gpu;

use lib::field::FieldError;
use lib::proto::{Msg, MsgCommand};
use lib::stream::StreamError;

#[derive(Debug)]
pub enum ComponentError {
    FieldError(String),  // Invalid field
    QueryError(String),  // Error during querying hardware
    OperationNotSupport, // Operation not supported by the hardware
    BadReply,
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

type Result<T> = std::result::Result<T, ComponentError>;

#[allow(unused_variables)]
pub trait Component {
    // Refresh self status from msg reply from daemon
    fn refresh_from_reply(&mut self, command: &MsgCommand, payload: &Option<Vec<u8>>)
    -> Result<()>;
}
