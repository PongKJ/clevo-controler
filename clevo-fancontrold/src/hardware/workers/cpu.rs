use crate::hardware::Hardware;
use clevo_fancontrol::domain::hw::field::usage::HwUsage;
use clevo_fancontrol::domain::hw::{
    self,
    field::hwname::{self, DeviceType},
};
use machine_info::Machine;

#[derive(Debug)]
pub struct Cpu {}

impl Hardware for Cpu {
    fn get_device_name() -> hwname::HwName {
        let mut m = Machine::new();
        let system_info = m.system_info();
        hwname::HwName::new(
            DeviceType::CPU,
            format!(
                "{} {}",
                system_info.processor.brand, system_info.processor.vendor
            ),
        )
    }

    fn get_device_info() -> hw::HwInfo {
        let mut m = Machine::new();
        let status = m.system_status().expect("Failed to get system_status");
        hw::HwInfo::new(HwUsage::new(0.0, status.cpu as f32, 0.0, 0.0, 0.0, 0.0))
    }
}
