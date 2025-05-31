pub mod intel;
use crate::lowlevel::accessor::fd;

#[derive(Debug, thiserror::Error)]
pub enum CpuError {
    #[error("get system info error")]
    SysInfoError,
    #[error("fd open error: {0}")]
    FdError(#[from] fd::FdError),
    #[error("fd not found")]
    FdNotFound,
    #[error("fd read date error")]
    FdReadError,
    #[error("parse int value error")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("request too frequent")]
    TooFrequent,
    #[error("timer interval too large, may overflow")]
    Overflow,
}

type Result<T> = std::result::Result<T, CpuError>;
