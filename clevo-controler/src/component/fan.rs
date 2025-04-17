use lib::field::fan_speed::FanIndex;
use lib::field::freq::TargetFreq;
use lib::field::{
    CpuStatus, fan_speed::FanSpeed, fan_speed::TargetFanSpeed, freq::Freq, power::Power,
    temp::Temp, usage::Usage,
};
use lib::proto::*;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

use crate::component::{Component, ComponentError};

pub struct Fan {
    id_num: u8,
    cpu_fan_speed: FanSpeed,
    gpu_fan_speed: FanSpeed,

    sender: Arc<Mutex<Sender<MsgBody>>>,
}

impl Fan {
    pub fn new(id_num: u8, sender: Arc<Mutex<Sender<MsgBody>>>) -> Self {
        Self {
            id_num,
            cpu_fan_speed: FanSpeed::default(),
            gpu_fan_speed: FanSpeed::default(),
            sender,
        }
    }

    pub fn get_cpu_fan_speed(&self) -> &FanSpeed {
        &self.cpu_fan_speed
    }

    pub fn get_gpu_fan_speed(&self) -> &FanSpeed {
        &self.gpu_fan_speed
    }

    pub fn set_fan_speed(&self, index: FanIndex, target_fan_speed: TargetFanSpeed) {
        let msg_packet = MsgPacket::new(
            MsgMode::Request,
            None,
            0,
            self.id_num,
            MsgCommand::SetFanSpeed,
        );
        let mut payload = vec![];
        payload.push(index.serialize().unwrap());
        payload.push(
            target_fan_speed
                .serialize()
                .expect("Failed to serialize payload"),
        );
        let msg_body = MsgBody::new(msg_packet, payload);
        let sender = self.sender.lock().unwrap();
        sender
            .send(msg_body)
            .expect("Failed to send message to the channel");
    }
}

impl Component for Fan {
    fn refresh_status(&mut self) -> super::Result<()> {
        let msg_packet = MsgPacket::new(
            MsgMode::Request,
            None,
            0,
            self.id_num,
            MsgCommand::GetFanSpeed,
        );
        let payload = FanIndex::All.serialize()?;
        let msg_body = MsgBody::new(msg_packet, vec![payload]);
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
        if *command == MsgCommand::GetFanSpeed {
            let fan_index: FanIndex =
                FanIndex::deserialize(&payload[0]).map_err(|e| ComponentError::BadReply)?;
            match fan_index {
                FanIndex::Cpu => {
                    self.cpu_fan_speed =
                        FanSpeed::deserialize(&payload[1]).map_err(|e| ComponentError::BadReply)?;
                }
                FanIndex::Gpu => {
                    self.gpu_fan_speed =
                        FanSpeed::deserialize(&payload[1]).map_err(|e| ComponentError::BadReply)?;
                }
                FanIndex::All => {
                    self.cpu_fan_speed =
                        FanSpeed::deserialize(&payload[1]).map_err(|e| ComponentError::BadReply)?;
                    self.gpu_fan_speed =
                        FanSpeed::deserialize(&payload[2]).map_err(|e| ComponentError::BadReply)?;
                }
            }
        }
        dbg!(&self.cpu_fan_speed);
        Ok(())
    }

    fn accept(&mut self, visitor: &mut dyn super::Visitor) {
        visitor.visit_fan(self);
    }
}
