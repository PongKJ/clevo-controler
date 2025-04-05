use crate::hardware::{Hardware, HardwareError};
use lib::field::HardwareList;
use lib::proto::*;
use lib::stream::{SocketStream, StreamListener};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

type Result<T> = std::result::Result<T, ProtoError>;

pub enum ServiceError {
    HardwareError(HardwareError),
    ProtoError(ProtoError),
}

pub struct ServiceConfig {
    socket_name: String,
}

pub struct Service {
    config: ServiceConfig,
    hardwares: Arc<Mutex<HashMap<u8, Box<dyn Hardware + Send + Sync>>>>,
}

impl Service {
    pub fn new(socket_name: &str) -> Result<Self> {
        Ok(Self {
            config: ServiceConfig {
                socket_name: socket_name.to_string(),
            },
            hardwares: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn add_hardware(
        &mut self,
        id: u8,
        hardware: Box<dyn Hardware + Send + Sync>,
    ) -> Result<()> {
        let mut hardwares = self.hardwares.lock().unwrap();
        hardwares.insert(id, hardware);
        Ok(())
    }

    pub fn spawn_monitor(&mut self) -> Result<JoinHandle<()>> {
        let hardwares_clone = Arc::clone(&self.hardwares);
        // thread to refresh the status of the hardware
        let handle = std::thread::spawn(move || {
            loop {
                let mut hardwares = hardwares_clone.lock().unwrap();
                hardwares.iter_mut().for_each(|(_, hardware)| {
                    hardware.refresh_status().unwrap();
                });
                drop(hardwares);
                std::thread::sleep(std::time::Duration::from_secs(3));
            }
        });
        Ok(handle)
    }
    pub fn spawn_msg_handler(&mut self) -> Result<JoinHandle<()>> {
        let socket_name = self.config.socket_name.clone();
        let hardwares_clone = Arc::clone(&self.hardwares);
        fn handle_msg(
            hardwares: &mut std::sync::MutexGuard<'_, HashMap<u8, Box<dyn Hardware + Send + Sync>>>,
            stream: &mut SocketStream,
            msg: Msg,
        ) {
            match msg.packet.command {
                MsgCommand::GetHardwareList => {
                    let mut hardware_list = HardwareList(HashMap::new());
                    hardwares.iter().for_each(|(id, hardware)| {
                        hardware_list.0.insert(*id, hardware.get_desc());
                    });
                    dbg!(&hardware_list);
                    let mut packet = msg.packet;
                    packet.mode = MsgMode::Reply;
                    let payload =
                        bincode::encode_to_vec(&hardware_list, bincode::config::standard())
                            .unwrap();
                    send_msg(stream, &packet, &Some(payload))
                        .expect("Failed to send hardware list reply");
                }
                _ => {
                    let hardware = hardwares.get_mut(&msg.packet.index).unwrap();
                    let payload_wrapped = hardware.handle_request(&msg);
                    if let Ok(payload) = payload_wrapped {
                        let mut packet = msg.packet;
                        packet.mode = MsgMode::Reply;
                        send_msg(stream, &packet, &payload).expect("Failed to send reply");
                    } else {
                        let payload = None;
                        let mut packet = msg.packet.clone();
                        packet.mode = MsgMode::Reply;
                        packet.error = Some(MsgError::UnsupportedOperation(format!(
                            "Operation not supported by the hardware:{}",
                            msg.packet.command
                        )));
                        send_msg(stream, &packet, &payload).expect("Failed to send reply");
                    }
                }
            }
        }
        let handle = std::thread::spawn(move || {
            let mut stream_listener = StreamListener::new(socket_name.as_str()).unwrap();
            loop {
                let mut stream = stream_listener
                    .accept()
                    .expect("Failed to accept stream connection");
                println!("Stream accepted, starting to handle requests...");
                loop {
                    match recv_msg(&mut stream) {
                        Ok(msg) => {
                            let mut hardwares = hardwares_clone.lock().unwrap();
                            handle_msg(&mut hardwares, &mut stream, msg);
                        }
                        Err(e) => {
                            println!("Error receiving message: {:?}", e);
                            break; // Exit the inner loop if there's an error
                        }
                    }
                }
            }
        });
        Ok(handle)
    }
}
