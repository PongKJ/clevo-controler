pub mod intel;
use crate::lowlevel::accessor::fd;

#[derive(Debug)]
pub enum CpuError {
    SysInfoError,
    FdError(fd::FdError),
    FdNotFound,
    ParseError,
    TooFrequent,
    Overflow,
}

impl From<CpuError> for String {
    fn from(err: CpuError) -> Self {
        match err {
            CpuError::SysInfoError => "SysInfoError".to_string(),
            CpuError::FdError(err) => format!("FdError: {}", err),
            CpuError::FdNotFound => "FdNotFound".to_string(),
            CpuError::ParseError => "ParseError".to_string(),
            CpuError::TooFrequent => "TooFrequent".to_string(),
            CpuError::Overflow => "Overflow".to_string(),
        }
    }
}

impl From<fd::FdError> for CpuError {
    fn from(err: fd::FdError) -> Self {
        CpuError::FdError(err)
    }
}

impl From<std::num::ParseIntError> for CpuError {
    fn from(_err: std::num::ParseIntError) -> Self {
        CpuError::ParseError
    }
}

type Result<T> = std::result::Result<T, CpuError>;
