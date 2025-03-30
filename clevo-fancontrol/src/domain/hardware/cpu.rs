use crate::communicate::proto::{HardwareIndex, Reply, Request};
use crate::communicate::query;
use crate::communicate::stream::StreamClient;
use crate::domain::hardware::field::{
    fan_speed::{FanSpeed, TargetFanSpeed},
    freq::{Freq, TargetFreq},
    power::{Power, TargetPower},
    temp::Temp,
    usage::Usage,
};

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

    fn handle_reply(&mut self, reply: Reply) {
        match reply {
            Reply::Desc(desc) => self.desc = desc,
            Reply::Freq(freq) => self.freq = freq,
            Reply::Usage(usage) => self.usage = usage,
            Reply::Temp(temp) => self.temp = temp,
            Reply::Power(power) => self.power = power,
            Reply::FanSpeed(fan_speed) => self.fan_speed = fan_speed,
            _ => (), // Ignore other replies
        }
    }
}

type Result<T> = std::result::Result<T, HardwareError>;

impl Hardware for Cpu {
    fn refresh(&mut self, stream_client: &mut StreamClient) -> Result<()> {
        self.handle_reply(query::send_query(
            stream_client,
            Request::GetFreq(HardwareIndex::Cpu),
        )?);
        self.handle_reply(query::send_query(
            stream_client,
            Request::GetFreq(HardwareIndex::Cpu),
        )?);
        self.handle_reply(query::send_query(
            stream_client,
            Request::GetUsage(HardwareIndex::Cpu),
        )?);
        self.handle_reply(query::send_query(
            stream_client,
            Request::GetTemp(HardwareIndex::Cpu),
        )?);
        self.handle_reply(query::send_query(
            stream_client,
            Request::GetPower(HardwareIndex::Cpu),
        )?);
        self.handle_reply(query::send_query(
            stream_client,
            Request::GetFanSpeed(HardwareIndex::Cpu),
        )?);
        Ok(())
    }

    fn get_desc(&self) -> &str {
        &self.desc
    }
    fn get_freq(&self) -> Result<&Freq> {
        Ok(&self.freq)
    }
    fn get_usage(&self) -> Result<&Usage> {
        Ok(&self.usage)
    }
    fn get_temp(&self) -> Result<&Temp> {
        Ok(&self.temp)
    }
    fn get_power(&self) -> Result<&Power> {
        Ok(&self.power)
    }
    fn get_fan_speed(&self) -> Result<&FanSpeed> {
        Ok(&self.fan_speed)
    }
}
