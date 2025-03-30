pub mod fan_speed;
pub mod freq;
pub mod identifier;
pub mod power;
pub mod temp;
pub mod usage;

#[derive(Debug)]
pub enum FieldError {
    InvalidValue(String), // Invalid value for a field
}

impl From<FieldError> for String {
    fn from(value: FieldError) -> Self {
        match value {
            FieldError::InvalidValue(msg) => format!("Invalid value: {}", msg),
        }
    }
}
