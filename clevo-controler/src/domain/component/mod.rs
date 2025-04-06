pub mod cpu;
pub mod gpu;

use lib::field::FieldError;
use lib::proto::Msg;
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
    // Refresh the hardware status
    fn refresh_status(&mut self) -> Result<Msg>;

    fn handle_reply(&mut self, msg: &Msg) -> Result<()>;
}
