use lib::field::{fan_speed::FanSpeed, freq::Freq, power::Power, temp::Temp, usage::Usage};
use lib::proto::*;

use crate::domain::component::{Component, ComponentError};

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

type Result<T> = std::result::Result<T, ComponentError>;

impl Component for Cpu {
    fn refresh_status(&mut self) -> super::Result<Msg> {
        todo!()
    }

    fn handle_reply(&mut self, msg: &Msg) -> super::Result<()> {
        todo!()
    }
}
