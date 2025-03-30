// TODO: Maybe rpc?
use crate::domain::field::fan_speed::{FanSpeed, TargetFanSpeed};
use crate::domain::field::freq::{Freq, TargetFreq};
use crate::domain::field::power::Power;
use crate::domain::field::temp::Temp;
use crate::domain::field::usage::Usage;
use serde::{Deserialize, Serialize};

pub trait Msg {
    fn get_raw_string(&self) -> String;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HardwareIndex {
    Cpu = 0,
    Gpu = 1,
}

// Each request is tagged with an index to identify the device
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Request {
    // Get
    GetDesc(HardwareIndex), // Get desciption of the device, tag this device by a index
    GetFreq(HardwareIndex),
    GetUsage(HardwareIndex),
    GetTemp(HardwareIndex),
    GetPower(HardwareIndex),
    GetFanSpeed(HardwareIndex),

    // Set
    SetFreq(HardwareIndex, TargetFreq), // Set frequency range in MHz (min,max)
    SetFanSpeed(HardwareIndex, TargetFanSpeed), // Set fan speed in percentage
}

impl From<Request> for String {
    fn from(request: Request) -> Self {
        serde_json::to_string(&request).unwrap() // Should not failed
    }
}

impl From<String> for Request {
    fn from(value: String) -> Self {
        serde_json::from_str(&value).unwrap() // Should not failed
    }
}

impl Msg for Request {
    fn get_raw_string(&self) -> String {
        String::from(self.clone())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Reply {
    // Reply with data
    Desc(String), // Description of the device
    Freq(Freq),   // in Mhz
    Usage(Usage), // in percentage
    Temp(Temp),   // in Celsius
    Power(Power), // in watt
    FanSpeed(FanSpeed),

    NotSupport,

    // Acknowledge
    Ack,
}

impl From<String> for Reply {
    fn from(reply: String) -> Self {
        serde_json::from_str(&reply).unwrap() //Should not failed
    }
}

impl From<Reply> for String {
    fn from(reply: Reply) -> Self {
        serde_json::to_string(&reply).unwrap() // Should not failed
    }
}

impl Msg for Reply {
    fn get_raw_string(&self) -> String {
        String::from(self.clone())
    }
}
