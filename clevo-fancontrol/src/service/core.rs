use crate::domain::hardware::Hardware;
use lib::field::HardwareList;
use lib::proto::ProtoError;
use lib::proto::recv_msg;
use lib::proto::send_msg;
use lib::proto::{MsgCommand, MsgMode, MsgPacket};
use lib::stream::SocketStream;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

type Result<T> = std::result::Result<T, ProtoError>;

pub struct ServiceConfig {
    interval: u64, // Time interval for refreshing hardware data by seconds
}

pub struct Service {
    config: ServiceConfig,
    hardwares: Arc<Mutex<Vec<Box<dyn Hardware + Send + Sync>>>>,
    socket_stream: Arc<Mutex<SocketStream>>,
}

impl Service {
    pub fn new(socket_name: &str) -> Result<Self> {
        let socket_stream = SocketStream::new(socket_name)?;
        Ok(Self {
            config: ServiceConfig {
                interval: 5, // Default refresh interval is 5 seconds
            },
            hardwares: Arc::new(Mutex::new(vec![])),
            socket_stream: Arc::new(Mutex::new(socket_stream)),
        })
    }
    pub fn add_hardware(&mut self, hardware: Box<dyn Hardware + Send + Sync>) -> Result<()> {
        let mut hardwares = self.hardwares.lock().unwrap();
        hardwares.push(hardware);
        Ok(())
    }

    pub fn spawn(&mut self) -> Result<JoinHandle<()>> {
        let hardwares_clone = Arc::clone(&self.hardwares);
        let socket_stream_clone = Arc::clone(&self.socket_stream);
        let interval = self.config.interval;
        let handle = std::thread::spawn(move || {
            // get hardware list first
            let packet = MsgPacket {
                mode: MsgMode::Request,
                error: None,
                sequence: 0,
                index: 0,
                command: MsgCommand::GetHardwareList,
            };
            send_msg(&mut socket_stream_clone.lock().unwrap(), &packet, &None).unwrap();
            let reply_msg = recv_msg(&mut socket_stream_clone.lock().unwrap()).unwrap();
            let (hardware_list, _): (HardwareList, _) = bincode::decode_from_slice(
                reply_msg.payload.as_deref().unwrap_or(&[]),
                bincode::config::standard(),
            )
            .unwrap();
            dbg!(&hardware_list);
            loop {
                let mut socket_stream_clone = socket_stream_clone.lock().unwrap();
                let packet = MsgPacket {
                    mode: MsgMode::Request,
                    error: None,
                    sequence: 0,
                    index: 0,
                    command: MsgCommand::GetStatus,
                };
                send_msg(&mut socket_stream_clone, &packet, &None);
                let msg = recv_msg(&mut socket_stream_clone).expect("Failed to receive message");
                dbg!(&msg);
                match msg.packet.command {
                    _ => {
                        // You can add logic to handle different commands
                        println!("Received command: {:?}", msg.packet.command);
                    }
                }
                let mut hardwares_clone = hardwares_clone.lock().unwrap();
                drop(socket_stream_clone);
                drop(hardwares_clone);
                std::thread::sleep(std::time::Duration::from_secs(interval));
            }
        });
        Ok(handle)
    }
}
