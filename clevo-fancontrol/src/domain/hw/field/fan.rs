use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum FanMode {
    Auto,
    Manual,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fan {
    pub mode: FanMode,
    pub rpm: u32,
    pub duty: f32,
}
