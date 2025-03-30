pub mod cpu;
pub mod gpu;

// pub trait Hardware {
//     fn get_desc(&self) -> String;
//     fn get_freq(&self, index: HardwareIndex) -> Freq;
//     fn get_temp(&self, index: HardwareIndex) -> Temp;
//     fn get_power(&self, index: HardwareIndex) -> Power;
//     fn get_usage(&self, index: HardwareIndex) -> Usage;
//     fn get_fan_speed(&self, index: HardwareIndex) -> FanSpeed;

// Optionally, you can define methods to set target values
// fn set_target_freq(&self, index: HardwareIndex, target_freq: TargetFreq);
// fn set_target_power(&self, index: HardwareIndex, target_power: TargetPower);
// fn set_target_fan_speed(&self, index: HardwareIndex, target_fan_speed: TargetFanSpeed);
// }
