use crate::{component::Component, lowlevel::accessor::ec};
use lib::field::{
    category::Category,
    fan_speed::{FanIndex, FanSpeed, TargetFanSpeed},
};

const EC_CPU_FAN_RPM_HI_ADDR: u8 = 0xD0;
const EC_CPU_FAN_RPM_LO_ADDR: u8 = 0xD1;
const EC_GPU_FAN_RPM_HI_ADDR: u8 = 0xD2;
const EC_GPU_FAN_RPM_LO_ADDR: u8 = 0xD3;
const EC_SET_FAN_SPEED_CMD: u8 = 0x99;
const EC_SET_FAN_AUTO_ADDR: u8 = 0xFF;

pub struct Fan {
    ec: ec::EcAccessor,
    cpu_fan_speed: FanSpeed,
    gpu_fan_speed: FanSpeed,
}

impl Default for Fan {
    fn default() -> Self {
        Self::new()
    }
}

impl Fan {
    pub fn new() -> Self {
        Fan {
            ec: ec::EcAccessor::new(),
            cpu_fan_speed: FanSpeed::default(),
            gpu_fan_speed: FanSpeed::default(),
        }
    }

    pub fn get_fan_rpm(&self, category: Category) -> u32 {
        let hi;
        let lo;
        match category {
            Category::Cpu => {
                hi = self.ec.read_byte(EC_CPU_FAN_RPM_HI_ADDR);
                lo = self.ec.read_byte(EC_CPU_FAN_RPM_LO_ADDR);
            }
            Category::Gpu => {
                hi = self.ec.read_byte(EC_GPU_FAN_RPM_HI_ADDR);
                lo = self.ec.read_byte(EC_GPU_FAN_RPM_LO_ADDR);
            }
            _ => {
                panic!("Invalid fan category");
            }
        }
        let rpm = ((hi as u16) << 8) | (lo as u16);
        if rpm == 0 { 0 } else { 2156220u32 / rpm as u32 }
    }

    pub fn set_fan_speed(&self, category: Category, duty: u64) {
        println!("Fan set_fan_speed duty: {}", duty);
        assert!(
            (0..=100).contains(&duty),
            "Duty cycle must be between 0 and 100"
        );
        self.ec.cmd_write(
            EC_SET_FAN_SPEED_CMD,
            category as u8,
            ((duty as f32 * 255.0) / 100.0) as u8,
        );
    }

    pub fn set_fan_auto(&self, category: Category) {
        println!("Fan set_fan_auto");
        self.ec
            .cmd_write(EC_SET_FAN_SPEED_CMD, EC_SET_FAN_AUTO_ADDR, category as u8);
    }
}

impl Component for Fan {
    fn get_desc(&self) -> lib::field::desc::Desc {
        lib::field::desc::Desc::new(Category::Fan, 0, "Fan")
    }
    fn refresh_status(&mut self) -> Result<(), crate::component::ComponentError> {
        self.cpu_fan_speed.set_rpm(self.get_fan_rpm(Category::Cpu));
        self.gpu_fan_speed.set_rpm(self.get_fan_rpm(Category::Gpu));
        Ok(())
    }
    fn handle_command(
        &mut self,
        command: &lib::proto::MsgCommand,
        payload: &Vec<Vec<u8>>,
    ) -> Result<Vec<Vec<u8>>, lib::proto::MsgError> {
        let mut reply_payload = vec![];
        reply_payload.extend_from_slice(payload);
        if !payload.is_empty() {
            let fan_index = FanIndex::deserialize(&payload[0]).unwrap();
            match command {
                lib::proto::MsgCommand::GetFanSpeed => match fan_index {
                    FanIndex::All => {
                        reply_payload.push(self.cpu_fan_speed.serialize().unwrap());
                        reply_payload.push(self.gpu_fan_speed.serialize().unwrap());
                    }
                    FanIndex::Cpu => {
                        reply_payload.push(self.cpu_fan_speed.serialize().unwrap());
                    }
                    FanIndex::Gpu => {
                        reply_payload.push(self.gpu_fan_speed.serialize().unwrap());
                    }
                },
                lib::proto::MsgCommand::SetFanSpeed => match fan_index {
                    FanIndex::Cpu => {
                        let target_fan_speed = TargetFanSpeed::deserialize(&payload[1]).unwrap();
                        self.set_fan_speed(Category::Cpu, target_fan_speed.get_duty() as u64);
                    }
                    FanIndex::Gpu => {
                        let target_fan_speed = TargetFanSpeed::deserialize(&payload[1]).unwrap();
                        self.set_fan_speed(Category::Gpu, target_fan_speed.get_duty() as u64);
                    }
                    FanIndex::All => {
                        let cpu_target_fan_speed =
                            TargetFanSpeed::deserialize(&payload[1]).unwrap();
                        let gpu_target_fan_speed =
                            TargetFanSpeed::deserialize(&payload[2]).unwrap();
                        self.set_fan_speed(Category::Cpu, cpu_target_fan_speed.get_duty() as u64);
                        self.set_fan_speed(Category::Gpu, gpu_target_fan_speed.get_duty() as u64);
                    }
                },
                lib::proto::MsgCommand::SetFanAuto => match fan_index {
                    FanIndex::Cpu => {
                        self.set_fan_auto(Category::Cpu);
                    }
                    FanIndex::Gpu => {
                        self.set_fan_auto(Category::Gpu);
                    }
                    FanIndex::All => {
                        self.set_fan_auto(Category::Cpu);
                        self.set_fan_auto(Category::Gpu);
                    }
                },
                _ => {}
            }
        } else {
            panic!("Attempt to communicate with 'fan' withot index payload ");
        }
        Ok(reply_payload)
    }
}
