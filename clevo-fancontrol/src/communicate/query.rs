use super::{
    proto::{Reply, Request},
    stream::StreamError,
};
use crate::{
    communicate::proto::HardwareIndex,
    communicate::stream::StreamClient,
    domain::field::{
        FieldError,
        fan_speed::{FanSpeed, TargetFanSpeed},
        freq::{Freq, TargetFreq},
        identifier::Identifier,
        power::{Power, TargetPower},
        temp::Temp,
        usage::Usage,
    },
};

#[derive(Debug)]
pub enum QueryError {
    StreamError(String),
    NotSupport,           // The requested operation is not supported by the device
    ErrorReply(String),   // Error message from the reply
    InvalidValue(String), // Invalid value received or expected
}

impl From<StreamError> for QueryError {
    fn from(err: StreamError) -> Self {
        QueryError::StreamError(err.to_string())
    }
}
impl From<FieldError> for QueryError {
    fn from(err: FieldError) -> Self {
        QueryError::InvalidValue(String::from(err))
    }
}

impl From<QueryError> for String {
    fn from(value: QueryError) -> Self {
        match value {
            QueryError::StreamError(msg) => format!("Stream error: {}", msg),
            QueryError::NotSupport => "Operation not supported".to_string(),
            QueryError::ErrorReply(msg) => format!("Error reply: {}", msg),
            QueryError::InvalidValue(msg) => format!("Invalid value: {}", msg),
        }
    }
}

type Result<T> = std::result::Result<T, QueryError>;

pub fn send_query(stream_client: &mut StreamClient, request: Request) -> Result<Reply> {
    stream_client.write(request)?;
    let reply_string = stream_client.read()?;
    if reply_string.is_empty() {
        return Err(QueryError::StreamError("Received empty reply".to_string()));
    }
    Ok(Reply::from(reply_string))
}

type QueryHandler = Box<dyn FnOnce(Request) -> Reply>;

pub fn reply_query(stream_client: &mut StreamClient, handler: QueryHandler) -> Result<()> {
    let request = Request::from(stream_client.read()?);
    dbg!(request.clone());
    let reply = handler(request);
    dbg!(reply.clone());
    stream_client.write(reply)?;
    Ok(())
}
