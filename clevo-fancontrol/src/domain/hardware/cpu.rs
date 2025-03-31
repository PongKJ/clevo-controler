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
            MsgCommand::GetCpuDesc => {
                self.desc = data.clone();
                Ok(())
            }
            MsgCommand::GetCpuFreq => {
                let freq = Freq::try_from(data.as_bytes())?;
                self.freq = freq;
                Ok(())
            }
            MsgCommand::GetCpuUsage => {
                let usage = Usage::try_from(data.as_bytes())?;
                self.usage = usage;
                Ok(())
            }
            MsgCommand::GetCpuTemp => {
                let temp = Temp::try_from(data.as_bytes())?;
                self.temp = temp;
                Ok(())
            }
            MsgCommand::GetCpuPower => {
                let power = Power::try_from(data.as_bytes())?;
                self.power = power;
                Ok(())
            }
            MsgCommand::GetCpuFanSpeed => {
                let fan_speed = FanSpeed::try_from(data.as_bytes())?;
                self.fan_speed = fan_speed;
                Ok(())
            }
            _ => Err(HardwareError::BadReply),
        }
    }
}
