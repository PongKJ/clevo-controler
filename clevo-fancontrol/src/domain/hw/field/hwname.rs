use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeviceType {
    CPU,
    GPU,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HwName {
    // Currently Only support cpu, gpu
    device_type: DeviceType,
    desc: String,
}

impl HwName {
    pub fn new(device_type: DeviceType, desc: String) -> Self {
        Self { device_type, desc }
    }

    pub fn get_device_type(&self) -> DeviceType {
        self.device_type.clone()
    }

    pub fn get_desc(&self) -> &String {
        &self.desc
    }
}
