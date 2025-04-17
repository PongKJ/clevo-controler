use lib::field::freq::TargetFreq;
use lib::field::{
    CpuStatus, fan_speed::FanSpeed, fan_speed::TargetFanSpeed, freq::Freq, power::Power,
    temp::Temp, usage::Usage,
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
            sender,
        }
    }

    pub fn set_freq(&self, target_freq: TargetFreq) {
        let msg_packet =
            MsgPacket::new(MsgMode::Request, None, 0, self.id_num, MsgCommand::SetFreq);
        let payload = target_freq
            .serialize()
            .expect("Failed to serialize payload");

        let msg_body = MsgBody::new(msg_packet, vec![payload]);
        let sender = self.sender.lock().unwrap();
        sender
            .send(msg_body)
            .expect("Failed to send message to the channel");
    }

    pub fn get_freq(&self) -> &Freq {
        &self.freq
    }
    pub fn get_usage(&self) -> &Usage {
        &self.usage
    }
    pub fn get_temp(&self) -> &Temp {
        &self.temp
    }
    pub fn get_power(&self) -> &Power {
        &self.power
    }
    pub fn get_desc(&self) -> &String {
        &self.desc
    }
}

type Result<T> = std::result::Result<T, ComponentError>;

impl Component for Cpu {
    fn refresh_status(&mut self) -> super::Result<()> {
        let msg_packet = MsgPacket::new(
            MsgMode::Request,
            None,
            0,
            self.id_num,
            MsgCommand::GetStatus,
        );
        let msg_body = MsgBody::new(msg_packet, vec![]);
        let sender = self.sender.lock().unwrap();
        sender
            .send(msg_body)
            .expect("Failed to send message to the channel");
        Ok(())
    }
    fn update_from_reply(
        &mut self,
        command: &MsgCommand,
        payload: &Vec<Vec<u8>>,
    ) -> super::Result<()> {
        match command {
            MsgCommand::GetStatus => {
                assert_eq!(payload.len(), 1);
                let cpu_status: CpuStatus =
                    CpuStatus::deserialize(&payload[0]).map_err(|e| ComponentError::BadReply)?;
                self.freq = cpu_status.freq;
                self.usage = cpu_status.usage;
                self.temp = cpu_status.temp;
                self.power = cpu_status.power;
                // let msg_packet =
                //     MsgPacket::new(MsgMode::Request, None, 0, self.id_num, MsgCommand::SetFreq)
                //         .serialize()
                //         .expect("Failed to serialize MsgPacket");
                // let sender = self.sender.lock().unwrap();
                // sender
                //     .send(MsgBody::new(msg_packet, Some(vec![1, 2, 2])))
                //     .unwrap();
            }
            MsgCommand::SetFreq => {}
            _ => {}
        }
        Ok(())
    }

    fn accept(&mut self, visitor: &mut dyn super::Visitor) {
        visitor.visit_cpu(self);
    }
}
