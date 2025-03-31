use lib::field::{
    fan_speed::{FanSpeed, TargetFanSpeed},
    freq::{Freq, TargetFreq},
    power::{Power, TargetPower},
    temp::Temp,
    usage::Usage,
};

#[derive(Debug)]
pub struct Cpu {}

impl Cpu {
    pub fn new() -> Self {
        Cpu {}
    }
}
