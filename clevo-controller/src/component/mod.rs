pub mod cpu;
pub mod fan;
pub mod gpu;

use cpu::Cpu;
use fan::Fan;
use gpu::Gpu;
use lib::field::FieldError;
use lib::proto::MsgCommand;
use lib::stream::StreamError;

#[derive(Debug, thiserror::Error)]
pub enum ComponentError {
    #[error("Field Error: {0}")]
    FieldError(#[from] FieldError), // Invalid field
    #[error("Query Error: {0}")]
    QueryError(#[from] StreamError), // Error during querying hardware
    #[error("Component not found")]
    OperationNotSupport, // Operation not supported by the hardware
    #[error("Bad reply from daemon")]
    BadReply,
}

type Result<T> = std::result::Result<T, ComponentError>;

#[allow(unused_variables)]
pub trait Component {
    // Refresh self status from msg reply from daemon
    fn refresh_status(&mut self) -> Result<()>;
    fn update_from_reply(&mut self, command: &MsgCommand, payload: &[Vec<u8>]) -> Result<()>;
    fn accept(&mut self, visitor: &mut dyn Visitor);
}

// 访问者模式, see https://en.wikipedia.org/wiki/Visitor_pattern,https://colobu.com/rust-patterns/patterns/behavioural/visitor.html
pub trait Visitor {
    fn visit_cpu(&mut self, cpu: &Cpu);
    fn visit_fan(&mut self, fan: &Fan);
    fn visit_gpu(&mut self, gpu: &Gpu);
}
