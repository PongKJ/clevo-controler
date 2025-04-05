// TODO: Remove this file
use lib::field::{
    fan_speed::{FanSpeed, TargetFanSpeed},
    freq::{Freq, TargetFreq},
    power::{Power, TargetPower},
    temp::Temp,
    usage::Usage,
};

// pub fn handle_get_desc(index: HardwareIndex) -> Reply {
//     let desc = "Clevo Fan Control Device".to_string();
//     Reply::Desc(desc)
// }
//
// pub fn handle_get_freq(index: HardwareIndex) -> Reply {
//     let freq = Freq::new(2400);
//     Reply::Freq(freq)
// }
//
// pub fn handle_get_temp(index: HardwareIndex) -> Reply {
//     let temp = Temp::new(60.0); // Placeholder for actual temperature in Celsius
//     Reply::Temp(temp)
// }
//
// pub fn handle_get_power(index: HardwareIndex) -> Reply {
//     let power = Power::new(65.0); // Placeholder for actual power in Watts
//     Reply::Power(power)
// }
//
// pub fn handle_get_usage(index: HardwareIndex) -> Reply {
//     let usage = Usage::new(75); // Placeholder for actual usage percentage (0.0 to 1.0)
//     Reply::Usage(usage)
// }
//
// pub fn handle_get_fan_speed(index: HardwareIndex) -> Reply {
//     let fan_speed = FanSpeed::new(50, 2000); // Placeholder for actual fan speed (0.0 to 1.0)
//     Reply::FanSpeed(fan_speed)
// }
