use crate::hardware::{Hardware, HardwareError};
use lib::field::HardwareList;
use lib::proto::*;
use lib::stream::StreamListener;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

type Result<T> = std::result::Result<T, ProtoError>;

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
                            match msg.packet.command {
                                MsgCommand::GetHardwareList => {
                                    let mut payload = HardwareList(HashMap::new());
                                    hardwares.iter().for_each(|(id, hardware)| {
                                        payload.0.insert(*id, hardware.get_desc());
                                    });
                                    dbg!(&payload);
                                    let mut packet = msg.packet;
                                    packet.mode = MsgMode::Reply;
                                    let payload = bincode::encode_to_vec(
                                        &payload,
                                        bincode::config::standard(),
                                    )
                                    .unwrap();
                                    send_msg(&mut stream, &packet, &Some(payload))
                                        .expect("Failed to send hardware list reply");
                                }
                                MsgCommand::GetStatus => {
                                    let hardware = hardwares.get_mut(&msg.packet.index).unwrap();
                                    let payload = hardware.handle_request(&msg).unwrap();
                                    let mut packet = msg.packet;
                                    packet.mode = MsgMode::Reply;
                                    send_msg(&mut stream, &packet, &payload)
                                        .expect("Failed to send CPU status reply");
                                }
                                _ => {
                                    println!("Received command: {:?}", msg.packet.command);
                                    // You can add logic to handle other commands
                                }
                            }
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
