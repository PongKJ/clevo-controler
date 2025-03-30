use clevo_fancontrol::{
    communicate::proto::HardwareIndex,
    domain::hardware::field::{
        fan_speed::{FanSpeed, TargetFanSpeed},
        freq::{Freq, TargetFreq},
        power::{Power, TargetPower},
        temp::Temp,
        usage::Usage,
    },
};

#[derive(Debug)]
pub struct Cpu {}

impl Cpu {
    pub fn new() -> Self {
        Cpu {}
    }
    
}
