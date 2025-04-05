use lib::field::{
    CpuStatus,
    fan_speed::{FanSpeed, TargetFanSpeed},
    freq::{Freq, TargetFreq},
    power::{Power, TargetPower},
    temp::Temp,
    usage::Usage,
};

use super::{Hardware, HardwareError};

#[derive(Debug, Default)]
struct Workload {
    pub freq_usage: bool,
    pub temp: bool,
    pub power: bool,
    pub fan_speed: bool,
}

#[derive(Debug)]
pub struct Cpu {
    desc: String,
    freq: Vec<Freq>,
    usage: Vec<Usage>,
    temp: Temp,
    power: Power,
    fan_speed: FanSpeed,
    workload: Workload,

    sysinfo: sysinfo::System,
}

impl Cpu {
    pub fn new() -> Self {
        let mut sysinfo = sysinfo::System::new();
        sysinfo.refresh_cpu_all();
        let desc = format!(
            "{}:{}",
            sysinfo.cpus()[0].vendor_id(),
            sysinfo.cpus()[0].brand(),
        );
        Cpu {
            desc,
            freq: sysinfo
                .cpus()
                .iter()
                .map(|cpu| Freq::new(cpu.frequency() as u32))
                .collect(),
            usage: sysinfo
                .cpus()
                .iter()
                .map(|cpu| Usage::new(cpu.cpu_usage() as u32))
                .collect(),
            temp: Temp::default(),
            power: Power::default(),
            fan_speed: FanSpeed::default(),
            workload: Workload {
                freq_usage: true,
                temp: true,
                power: true,
                fan_speed: true,
            },

            sysinfo,
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
        if self.workload.freq_usage {
            self.sysinfo.refresh_cpu_all();
            self.freq.iter_mut().for_each(|cpu| {
                cpu.set_value(self.sysinfo.cpus()[0].cpu_usage() as u32);
            });
            self.usage.iter_mut().for_each(|cpu| {
                cpu.set_value(self.sysinfo.cpus()[0].cpu_usage() as u32);
            });
        }
        if self.workload.temp {
            // TODO: Get temp
            self.temp = Temp::new(50.0);
        }
        // if self.workload.

        Ok(())
    }

    fn handle_request(&mut self, msg: &Msg) -> Result<Option<Vec<u8>>, MsgError> {
        let mut payload = None;
        match msg.packet.command {
            MsgCommand::GetStatus => {
                let cpu_status = CpuStatus {
                    freq: self.freq.clone(),
                    power: self.power.clone(),
                    temp: self.temp.clone(),
                    usage: self.usage.clone(),
                    fan_speed: self.fan_speed.clone(),
                };
                payload = Some(cpu_status);
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
