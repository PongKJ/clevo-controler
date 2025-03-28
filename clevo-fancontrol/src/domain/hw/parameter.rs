use crate::domain::hw::field::fan::Fan;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SetDeviceParam {
    pub fan: Fan,
}

impl SetDeviceParam {
    pub fn new(fan: Fan) -> Self {
        Self { fan }
    }
}
