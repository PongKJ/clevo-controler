pub mod core;

use lib::proto::ProtoError;

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Socket Error: {0}")]
    SocketError(#[from] ProtoError),
}
