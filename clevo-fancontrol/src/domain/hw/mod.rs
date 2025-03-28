use serde::{Deserialize, Serialize};

pub mod ctrler;
pub mod field;
pub mod parameter;

#[derive(Debug, Serialize, Deserialize)]
pub struct HwInfo {
    usage: field::usage::HwUsage,
}

impl HwInfo {
    pub fn new(usage: field::usage::HwUsage) -> Self {
        Self { usage }
    }
}

pub struct HwCtrl {
    hw_info: Vec<HwInfo>,
    ctrler: ctrler::HwCtrl,
}

impl HwCtrl {
    pub fn new(ctrler: ctrler::HwCtrl) -> Self {
        Self {
            hw_info: vec![],
            ctrler,
        }
    }
    pub fn run(&mut self) {
        self.ctrler.run();
    }
}
