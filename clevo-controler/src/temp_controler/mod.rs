use lib::field::{
    category::Category,
    fan_speed::{FanIndex, TargetFanSpeed},
    temp::Temp,
};
use pid::PidControler;
use serde::{Deserialize, Serialize};

use crate::component::Visitor;

pub mod pid;
pub mod range;
pub mod table;

#[derive(Debug, Deserialize, Serialize)]
pub enum Method {
    TableLookUp,
    TempRange,
    Pid,
}
#[derive(Debug)]
pub enum ControlerError {
    InvalidTemp,
    InvalidFanSpeed,
    InvalidMethod,
    NotSupport,
}

pub trait ControlerAlgo {
    // return fan speed in percentage(1~100)
    fn update(&mut self, current_temp: &Temp) -> u32;
}

pub struct Controler {
    cfg_path: String,
    cfg: ControlerCfg,

    cpu_algo: Box<dyn ControlerAlgo>,
    cpu_current_temp: Temp,

    gpu_algo: Box<dyn ControlerAlgo>,
    gpu_current_temp: Temp,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ControlerCfg {
    pub cpu_method: Method,
    pub gpu_method: Method,

    pub cpu_pid_cfg: pid::PidCfg,
    pub gpu_pid_cfg: pid::PidCfg,
}

impl Default for ControlerCfg {
    fn default() -> Self {
        ControlerCfg {
            cpu_method: Method::Pid,
            cpu_pid_cfg: pid::PidCfg {
                target_temp: 60000.0,
                kp: 1.0,
                ki: 0.5,
                kd: 0.5,
            },

            gpu_method: Method::Pid,
            gpu_pid_cfg: pid::PidCfg {
                target_temp: 60000.0,
                kp: 1.0,
                ki: 0.5,
                kd: 0.5,
            },
        }
    }
}

impl Controler {
    pub fn new(cfg_path: &str) -> Self {
        let mut controler = Controler {
            cfg_path: cfg_path.to_string(),
            cfg: ControlerCfg::default(),

            cpu_algo: Box::new(PidControler::new(ControlerCfg::default().cpu_pid_cfg)),
            cpu_current_temp: Temp::default(),

            gpu_algo: Box::new(PidControler::new(ControlerCfg::default().gpu_pid_cfg)),
            gpu_current_temp: Temp::default(),
        };
        if std::fs::exists(cfg_path).is_ok() {
            controler.load_from_json();
        }
        controler
    }

    pub fn load_from_json(&mut self) {
        // read json config file
        // parse json to Controler
        let cfg = std::fs::read_to_string(&self.cfg_path).unwrap();
        let cfg: ControlerCfg = serde_json::from_str(&cfg).unwrap();
        match cfg.cpu_method {
            Method::TableLookUp => {
                unimplemented!()
            }
            Method::TempRange => {
                unimplemented!()
            }
            Method::Pid => {
                let pid = pid::PidControler::new(cfg.cpu_pid_cfg);
                self.cpu_algo = Box::new(pid);
            }
        }
        // TODO: load gpu algo
    }

    pub fn save_to_json(&self) {
        // write json config file
        // parse Controler to json
        let cfg = serde_json::to_string(&self.cfg).unwrap();
        std::fs::write(&self.cfg_path, cfg).unwrap();
    }
}

impl Drop for Controler {
    fn drop(&mut self) {
        // save to json
        self.save_to_json();
    }
}

impl Visitor for Controler {
    fn visit_cpu(&mut self, cpu: &crate::component::cpu::Cpu) {
        // 访问 CPU 组件
        println!("Visiting CPU: {:#?}", cpu);
        // 在这里可以执行一些操作，例如获取 CPU 的频率、温度等信息
        // self.expect_temp = cpu.get_temp();
    }
    fn visit_fan(&mut self, fan: &crate::component::fan::Fan) {
        let cpu_target_fan_speed = self.cpu_algo.update(&self.cpu_current_temp);
        dbg!(&cpu_target_fan_speed);
        fan.set_fan_speed(FanIndex::Cpu, TargetFanSpeed::new(cpu_target_fan_speed));
    }
}
