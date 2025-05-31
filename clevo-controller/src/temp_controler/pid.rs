// use pid to control fan speed
use lib::field::temp::Temp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PidCfg {
    pub target_temp: f32,
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,

    pub smoothing_factor: f32,
}

pub struct PidControler {
    cfg: PidCfg,
    prev_error: f32,
    integral: f32,

    last_update_time: std::time::SystemTime,
    prev_speed: Option<f32>,
}

impl PidControler {
    pub fn new(cfg: PidCfg) -> Self {
        PidControler {
            cfg,
            prev_error: 0.0,
            integral: 0.0,
            last_update_time: std::time::SystemTime::now() - std::time::Duration::from_secs(2),
            prev_speed: None,
        }
    }
}

use super::ControlerAlgo;
impl ControlerAlgo for PidControler {
    fn update(&mut self, current_temp: &Temp) -> u32 {
        let current_temp = current_temp.get_value() as f32;
        let delta_time = self.last_update_time.elapsed().unwrap().as_secs_f32();
        self.last_update_time = std::time::SystemTime::now();
        let error = (current_temp - self.cfg.target_temp).clamp(0.0, 8000.0);
        self.integral += error * delta_time;
        self.integral = self.integral.clamp(0.0, 2000.0);
        let derivative = (error - self.prev_error) / delta_time;
        self.prev_error = error;
        let raw_output =
            self.cfg.kp * error + self.cfg.ki * self.integral + self.cfg.kd * derivative;
        let raw_speed = (raw_output / 100.0).clamp(0.0, 100.0);
        let smoothed_speed = if let Some(prev_speed) = self.prev_speed {
            prev_speed + (raw_speed - prev_speed) * self.cfg.smoothing_factor
        } else {
            raw_speed
        };
        self.prev_speed = Some(smoothed_speed);
        smoothed_speed as u32
    }
}
