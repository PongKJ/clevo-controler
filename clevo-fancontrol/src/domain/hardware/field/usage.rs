use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Usage {
    value: u32, // CPU usage in percentage
}

impl Usage {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
    pub fn get_value(&self) -> u32 {
        self.value
    }
}
