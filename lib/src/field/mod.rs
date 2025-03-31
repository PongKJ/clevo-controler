pub mod fan_speed;
pub mod freq;
// pub mod identifier;
pub mod power;
pub mod temp;
pub mod usage;
use std::fmt::Display;

#[derive(Debug)]
pub enum FieldError {
    InvalidValue(String), // Invalid value for a field
    ParseError(String),   // Error during parsing
}

impl Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            FieldError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl From<FieldError> for String {
    fn from(value: FieldError) -> Self {
        match value {
            FieldError::InvalidValue(msg) => format!("Invalid value: {}", msg),
            FieldError::ParseError(msg) => format!("Parse error: {}", msg),
        }
    }
}

impl From<bincode::error::DecodeError> for FieldError {
    fn from(err: bincode::error::DecodeError) -> Self {
        FieldError::ParseError(format!("decode error: {}", err))
    }
}

impl From<bincode::error::EncodeError> for FieldError {
    fn from(err: bincode::error::EncodeError) -> Self {
        FieldError::ParseError(format!("encode error: {}", err))
    }
}
