use lib::field::{fan_speed::FanSpeed, freq::Freq, power::Power, temp::Temp, usage::Usage};
use lib::proto::*;

use crate::domain::hardware::Hardware;
use crate::domain::hardware::HardwareError;

#[derive(Debug, Default)]
pub struct Cpu {
    desc: String,
    freq: Freq,
    usage: Usage,
    temp: Temp,
    power: Power,
    fan_speed: FanSpeed,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }
}

type Result<T> = std::result::Result<T, HardwareError>;

impl Hardware for Cpu {
    fn refresh_status(&mut self, command: &MsgCommand, data: &String) -> Result<()> {
        match command {
            MsgCommand::GetStatus => {
                self.desc = data.clone();
                Ok(())
            }
            _ => Err(HardwareError::BadReply),
        }
    }
}
