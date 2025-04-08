use lib::field::freq::TargetFreq;
use lib::field::{
    CpuStatus, fan_speed::FanSpeed, freq::Freq, power::Power, temp::Temp, usage::Usage,
};
use lib::proto::*;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::component::{Component, ComponentError};

#[derive(Debug)]
pub struct Cpu {
    id_num: u8,
    desc: String,
    freq: Freq,
    usage: Usage,
    temp: Temp,
    power: Power,
    fan_speed: FanSpeed,

    sender: Arc<Mutex<Sender<MsgBody>>>,
}

impl Cpu {
    pub fn new(id_num: u8, sender: Arc<Mutex<Sender<MsgBody>>>) -> Self {
        Self {
            id_num,
            desc: String::default(),
            freq: Freq::default(),
            usage: Usage::default(),
            temp: Temp::default(),
            power: Power::default(),
            fan_speed: FanSpeed::default(),
            sender,
        }
    }

    pub fn set_freq(&self, target_freq: TargetFreq) {
        let msg_packet =
            MsgPacket::new(MsgMode::Request, None, 0, self.id_num, MsgCommand::SetFreq)
                .serialize()
                .expect("Failed to serialize MsgPacket");
        let payload = target_freq
            .serialize()
            .expect("Failed to serialize payload");

        let msg_body = MsgBody::new(msg_packet, Some(payload));
        let sender = self.sender.lock().unwrap();
        sender
            .send(msg_body)
            .expect("Failed to send message to the channel");
    }
}

type Result<T> = std::result::Result<T, ComponentError>;

impl Component for Cpu {
    fn refresh_from_reply(
        &mut self,
        command: &MsgCommand,
        payload: &Option<Vec<u8>>,
    ) -> super::Result<()> {
        match command {
            MsgCommand::GetStatus => {
                if let Some(payload) = payload {
                    let cpu_status: CpuStatus =
                        CpuStatus::deserialize(payload).map_err(|e| ComponentError::BadReply)?;
                    self.freq = cpu_status.freq;
                    self.usage = cpu_status.usage;
                    self.temp = cpu_status.temp;
                    self.power = cpu_status.power;
                    self.fan_speed = cpu_status.fan_speed;
                }
                let msg_packet =
                    MsgPacket::new(MsgMode::Request, None, 0, self.id_num, MsgCommand::SetFreq)
                        .serialize()
                        .expect("Failed to serialize MsgPacket");
                let sender = self.sender.lock().unwrap();
                sender.send(MsgBody::new(msg_packet, Some(vec![1,2,2]))).unwrap();
            }
            _ => {}
        }
        Ok(())
    }

    fn accept(&mut self, visitor: &mut dyn super::Visitor) {
        visitor.visit_cpu(self);
    }
}
