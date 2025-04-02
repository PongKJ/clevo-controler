use lib::field::{
    fan_speed::{FanSpeed, TargetFanSpeed},
    freq::{Freq, TargetFreq},
    power::{Power, TargetPower},
    temp::Temp,
    usage::Usage,
};

use super::{Hardware, HardwareError};

#[derive(Debug, Default)]
struct Workload {
    pub freq: bool,
    pub temp: bool,
    pub power: bool,
    pub usage: bool,
    pub fan_speed: bool,
}

#[derive(Debug)]
pub struct Cpu {
    desc: String,
    freq: Freq,
    temp: Temp,
    power: Power,
    usage: Usage,
    fan_speed: FanSpeed,
    workload: Workload,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            desc: "TODO:get cpu vendor".to_string(),
            freq: Freq::default(),
            temp: Temp::default(),
            power: Power::default(),
            usage: Usage::default(),
            fan_speed: FanSpeed::default(),
            workload: Workload::default(),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu::new()
    }
}

use lib::proto::*;
impl Hardware for Cpu {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }

    fn refresh_status(&mut self) -> Result<(), HardwareError> {
        if self.workload.freq {
            // TODO: get cpu freq
            self.freq = Freq::new(2400);
        }

        Ok(())
    }

    fn handle_request(&mut self, msg: &Msg) -> Result<Option<Vec<u8>>, MsgError> {
        let mut payload = None;
        match msg.packet.command {
            MsgCommand::GetStatus => {
                payload = Some(self.desc.clone());
            }
            MsgCommand::SetFreq => {
                // do nothing
            }
            _ => {
                return Err(MsgError::UnsupportedOperation(format!(
                    "Operation not supported by the hardware:{}",
                    msg.packet.command
                )));
            }
        }
        if let Some(payload) = payload {
            Ok(Some(
                bincode::encode_to_vec(&payload, bincode::config::standard())
                    .map_err(|e| MsgError::ServerError(e.to_string()))?,
            ))
        } else {
            Ok(None)
        }
    }
}
