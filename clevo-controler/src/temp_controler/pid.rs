// use pid to control fan speed
use lib::field::{fan_speed::TargetFanSpeed, temp::Temp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PidCfg {
    pub target_temp: f32,
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,
}

pub struct PidControler {
    cfg: PidCfg,
    prev_error: f32,
    integral: f32,

    last_update_time: std::time::SystemTime,
}

impl PidControler {
    pub fn new(cfg: PidCfg) -> Self {
        PidControler {
            cfg,
            prev_error: 0.0,
            integral: 0.0,
            last_update_time: std::time::SystemTime::now(),
        }
    }
}

use super::ControlerAlgo;
impl ControlerAlgo for PidControler {
    fn update(&mut self, current_temp: &Temp) -> u32 {
        let current_temp = current_temp.get_value() as f32;
        let delta_time = self.last_update_time.elapsed().unwrap().as_secs_f32();
        self.last_update_time = std::time::SystemTime::now();
        let error = current_temp - self.cfg.target_temp;
        self.integral += error * delta_time;
        let derivative = (error - self.prev_error) / delta_time;
        self.prev_error = error;
        ((self.cfg.kp * error + self.cfg.ki * self.integral + self.cfg.kd * derivative) / 1000.0)
            as u32
    }
}
