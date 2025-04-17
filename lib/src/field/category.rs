use bincode::{Decode, Encode};

#[derive(Debug, Default, Clone, Encode, Decode)]
pub enum Category {
    #[default]
    Cpu = 1,
    Gpu,
    Fan,
}
