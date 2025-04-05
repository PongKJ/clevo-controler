use lib::proto::ProtoError;

pub mod core;

pub enum ServiceError {
    SocketError(String),
}

impl From<ProtoError> for ServiceError {
    fn from(err: ProtoError) -> Self {
        ServiceError::SocketError(format!("ProtoError occurred: {:?}", err))
    }
}
