use lib::{
    field::{freq::Freq, power::Power, temp::Temp, usage::Usage},
    proto::*,
};
use std::sync::{Arc, Mutex, mpsc::Sender};

#[allow(dead_code)]
pub struct Gpu {
    id_num: u8,
    desc: String,
    freq: Freq,
    usage: Usage,
    temp: Temp,
    power: Power,
    sender: Arc<Mutex<Sender<MsgBody>>>,
}
