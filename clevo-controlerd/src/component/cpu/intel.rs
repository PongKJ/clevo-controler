use crate::component::cpu::CpuError;
use crate::{component::Component, lowlevel::accessor::fd};
use std::collections::HashMap;

// TODO: Self impl instead of use sysinfo crates
#[derive(Debug)]
pub struct IntelCpu {
    sysinfo: sysinfo::System,
    fd_list: HashMap<String, fd::Fd>,
    last_refresh_time_stamp: std::time::Instant,
    energy_comsumption: u64,

    index: u8, // preserve, not use
    name: String,
    freq: Vec<u64>,
    usage: Vec<f32>,
    period_power: u64,
    temp: u64,
}

/// for example:
/// add_fd("/sys/class/powercap/intel-rapl:","name","package-0","energy_uj",5)
fn add_fd(
    fd_list: &mut HashMap<String, fd::Fd>,
    common_path: &str,
    key: &str,
    value: &str,
    key_to_add: &str,
    max_index: u8,
) {
    let iter = 0..max_index;
    for index in iter {
        let fd_path = format!("{}{}/{}", common_path, index, key);
        let fd = fd::Fd::new(&fd_path, libc::O_RDONLY);
        if let Ok(fd) = fd {
            let read_value = fd.read(32);
            if let Ok(read_value) = read_value {
                if read_value == value {
                    let fd_path = format!("{}{}/{}", common_path, index, key_to_add);
                    let fd = fd::Fd::new(&fd_path, libc::O_RDONLY);
                    if let Ok(fd) = fd {
                        fd_list.insert(key_to_add.to_string(), fd);
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
}

impl IntelCpu {
    pub fn init(index: u8) -> super::Result<Self> {
        let mut sysinfo = sysinfo::System::new();
        sysinfo.refresh_cpu_all();
        let mut cpu = IntelCpu {
            index,
            name: format!(
                "{}:{}",
                sysinfo.cpus()[0].vendor_id(),
                sysinfo.cpus()[0].brand()
            ),
            freq: sysinfo.cpus().iter().map(|cpu| cpu.frequency()).collect(),
            usage: sysinfo.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
            sysinfo,
            fd_list: HashMap::new(),
            energy_comsumption: 0,
            last_refresh_time_stamp: std::time::Instant::now(),
            period_power: 0,
            temp: 0,
        };
        add_fd(
            &mut cpu.fd_list,
            "/sys/class/powercap/intel-rapl:",
            "name",
            "package-0",
            "energy_uj",
            3,
        );
        add_fd(
            &mut cpu.fd_list,
            "/sys/class/thermal/thermal_zone",
            "type",
            "x86_pkg_temp",
            "temp",
            3,
        );

        cpu.energy_comsumption = match cpu.fd_list.get("energy_uj") {
            Some(fd) => fd.read(32)?.parse()?,
            None => {
                return Err(CpuError::FdNotFound);
            }
        };

        Ok(cpu)
    }

    pub fn refresh(&mut self) -> super::Result<()> {
        if self.last_refresh_time_stamp.elapsed().as_secs() < 1 {
            return Err(CpuError::TooFrequent);
        }
        if self.last_refresh_time_stamp.elapsed().as_secs() > 10 {
            return Err(CpuError::Overflow);
        }
        // refresh power
        let current_energy_comsumption = match self.fd_list.get("energy_uj") {
            Some(fd) => fd.read(32)?.parse()?,
            None => {
                return Err(CpuError::FdNotFound);
            }
        };
        self.period_power = (current_energy_comsumption - self.energy_comsumption)
            / self.last_refresh_time_stamp.elapsed().as_millis() as u64;
        self.energy_comsumption = current_energy_comsumption;
        self.last_refresh_time_stamp = std::time::Instant::now();

        // refresh cpu temperature
        self.temp = match self.fd_list.get("temp") {
            Some(fd) => fd.read(32)?.parse()?,
            None => {
                return Err(CpuError::FdNotFound);
            }
        };

        self.sysinfo.refresh_cpu_all();
        // refresh cpu usage
        self.usage = self
            .sysinfo
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();
        // refresh cpu frequency
        self.freq = self
            .sysinfo
            .cpus()
            .iter()
            .map(|cpu| cpu.frequency())
            .collect();
        Ok(())
    }
}

use lib::field::category::Category;
use lib::field::fan_speed::TargetFanSpeed;
use lib::field::{
    CpuStatus, desc::Desc, fan_speed::FanSpeed, freq::Freq, power::Power, temp::Temp, usage::Usage,
};
use lib::proto::{MsgCommand, MsgError};
impl Component for IntelCpu {
    fn get_desc(&self) -> Desc {
        Desc::new(Category::Cpu, self.index, &self.name)
    }

    fn refresh_status(&mut self) -> std::result::Result<(), crate::component::ComponentError> {
        self.refresh().map_err(|e| e.into())
    }
    fn handle_command(
        &mut self,
        command: &MsgCommand,
        payload: &Vec<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>, MsgError> {
        let mut reply_payload = vec![];
        match command {
            MsgCommand::GetStatus => {
                let cpu_status = CpuStatus {
                    freq: Freq::new(self.freq.clone()),
                    power: Power::new(self.period_power),
                    temp: Temp::new(self.temp),
                    usage: Usage::new(self.usage.clone()),
                };
                reply_payload.push(cpu_status.serialize().unwrap());
            }
            MsgCommand::SetFreq => {
                println!("SetFreq");
            }
            _ => {
                MsgError::UnsupportedOperation(format!(
                    "Operation not supported by the hardware:{}",
                    command
                ));
            }
        }
        Ok(reply_payload)
    }
}
