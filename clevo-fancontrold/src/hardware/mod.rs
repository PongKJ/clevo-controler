pub mod accessor;
pub mod peripheries;
pub mod workers;

use clevo_fancontrol::domain::hw::{self, field::hwname};

pub trait Hardware {
    fn get_device_name() -> hwname::HwName;
    fn get_device_info() -> hw::HwInfo;
}
