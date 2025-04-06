use crate::{
    component::Component,
    lowlevel::{accessor::fd, middleware},
};
use std::collections::HashMap;

#[derive(Debug)]
pub enum CpuError {
    SysInfoError,
    FdError(fd::FdError),
    FdNotFound,
    ParseError,
    TooFrequent,
    Overflow,
}

impl From<CpuError> for String {
    fn from(err: CpuError) -> Self {
        match err {
            CpuError::SysInfoError => "SysInfoError".to_string(),
            CpuError::FdError(err) => format!("FdError: {}", err),
            CpuError::FdNotFound => "FdNotFound".to_string(),
            CpuError::ParseError => "ParseError".to_string(),
            CpuError::TooFrequent => "TooFrequent".to_string(),
            CpuError::Overflow => "Overflow".to_string(),
        }
    }
}

impl From<fd::FdError> for CpuError {
    fn from(err: fd::FdError) -> Self {
        CpuError::FdError(err)
    }
}

impl From<std::num::ParseIntError> for CpuError {
    fn from(_err: std::num::ParseIntError) -> Self {
        CpuError::ParseError
    }
}

type Result<T> = std::result::Result<T, CpuError>;

// TODO: Self impl instead of use sysinfo crates
#[derive(Debug)]
pub struct Cpu {
    sysinfo: sysinfo::System,
    fd_list: HashMap<String, fd::Fd>,
    last_refresh_time_stamp: std::time::Instant,
    energy_comsumption: u64,

    pub desc: String,
    pub freq: Vec<u32>,
    pub usage: Vec<u32>,
    pub period_power: u64,
    pub temp: u64,
    pub fan_speed: u64,
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

impl Cpu {
    pub fn init() -> Result<Self> {
        let mut sysinfo = sysinfo::System::new();
        sysinfo.refresh_cpu_all();
        let mut cpu = Cpu {
            desc: format!(
                "{}:{}",
                sysinfo.cpus()[0].vendor_id(),
                sysinfo.cpus()[0].brand()
            ),
            freq: sysinfo
                .cpus()
                .iter()
                .map(|cpu| cpu.frequency() as u32)
                .collect(),
            usage: sysinfo
                .cpus()
                .iter()
                .map(|cpu| cpu.cpu_usage() as u32)
                .collect(),
            sysinfo,
            fd_list: HashMap::new(),
            energy_comsumption: 0,
            last_refresh_time_stamp: std::time::Instant::now(),
            period_power: 0,
            temp: 0,
            fan_speed: 0,
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

    pub fn refresh(&mut self) -> Result<()> {
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
            .map(|cpu| cpu.cpu_usage() as u32)
            .collect();
        // refresh cpu frequency
        self.freq = self
            .sysinfo
            .cpus()
            .iter()
            .map(|cpu| cpu.frequency() as u32)
            .collect();
        // refresh fan speed
        let fan = middleware::fan::Fan::get_instance();
        self.fan_speed = fan.get_fan_rpm(middleware::fan::FanIndex::CPU) as u64;
        Ok(())
    }
}

use lib::field::{
    CpuStatus, fan_speed::FanSpeed, freq::Freq, power::Power, temp::Temp, usage::Usage,
};
use lib::proto::{Msg, MsgCommand, MsgError};
impl Component for Cpu {
    fn get_desc(&self) -> String {
        self.desc.clone()
    }

    fn refresh_status(&mut self) -> std::result::Result<(), crate::component::ComponentError> {
        self.refresh().map_err(|e| e.into())
    }
    fn handle_request(&mut self, msg: &Msg) -> std::result::Result<Option<Vec<u8>>, MsgError> {
        let payload;
        match msg.packet.command {
            MsgCommand::GetStatus => {
                let cpu_status = CpuStatus {
                    freq: Freq::new(self.freq.clone()),
                    power: Power::new(self.period_power),
                    temp: Temp::new(self.temp),
                    usage: Usage::new(self.usage.clone()),
                    fan_speed: FanSpeed::new(self.fan_speed),
                };
                payload = Some(cpu_status);
            }
            MsgCommand::SetFreq => {
                todo!()
            }
            MsgCommand::SetFanSpeed => {
                todo!()
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
