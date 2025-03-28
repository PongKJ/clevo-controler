use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HwUsage {
    freq: f32,
    load: f32,
    temp: f32,
    // May not support to be detected
    power: f32,
    voltage: f32,
    current: f32,
}

impl HwUsage {
    pub fn new(freq: f32, load: f32, temp: f32, power: f32, voltage: f32, current: f32) -> Self {
        Self {
            freq,
            load,
            temp,
            power,
            voltage,
            current,
        }
    }
    pub fn get_freq(&self) -> f32 {
        self.freq
    }
    pub fn get_load(&self) -> f32 {
        self.load
    }
    pub fn get_temp(&self) -> f32 {
        self.temp
    }
    pub fn get_power(&self) -> f32 {
        self.power
    }
    pub fn get_voltage(&self) -> f32 {
        self.voltage
    }
    pub fn get_current(&self) -> f32 {
        self.current
    }
}
