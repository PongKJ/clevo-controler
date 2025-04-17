use crate::component::{Component, ComponentError};
use crate::lowlevel::accessor::ec::EcAccessor;
use lib::field::ComponentList;
use lib::proto::*;
use lib::stream::{SocketStream, StreamListener};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

type Result<T> = std::result::Result<T, ProtoError>;

pub enum ServiceError {
    ComponentError(ComponentError),
    ProtoError(ProtoError),
}

pub struct ServiceConfig {
    socket_name: String,
}

pub struct Service {
    config: ServiceConfig,
    components: Arc<Mutex<HashMap<u8, Box<dyn Component + Send + Sync>>>>,
}

impl Service {
    pub fn new(socket_name: &str) -> Result<Self> {
        Ok(Self {
            config: ServiceConfig {
                socket_name: socket_name.to_string(),
            },
            components: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    pub fn add_hardware(
        &mut self,
        id: u8,
        hardware: Box<dyn Component + Send + Sync>,
    ) -> Result<()> {
        let mut hardwares = self.components.lock().unwrap();
        hardwares.insert(id, hardware);
        Ok(())
    }

    pub fn spawn_monitor(&mut self) -> Result<JoinHandle<()>> {
        let hardwares_clone = Arc::clone(&self.components);
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
        let hardwares_clone = Arc::clone(&self.components);
        fn handle_msg(
            hardwares: &mut std::sync::MutexGuard<
                '_,
                HashMap<u8, Box<dyn Component + Send + Sync>>,
            >,
            stream: &mut SocketStream,
            body: MsgBody,
        ) -> Result<()> {
            let mut packet = body.get_packet().clone();
            packet.set_mode(MsgMode::Reply);
            let mut payload = vec![];
            // dbg!(&packet.get_command());
            match packet.get_command() {
                MsgCommand::GetComponentList => {
                    let mut hardware_list = ComponentList(HashMap::new());
                    hardwares.iter().for_each(|(id, hardware)| {
                        hardware_list.0.insert(*id, hardware.get_desc());
                    });
                    dbg!(&hardware_list);
                    payload.push(bincode::encode_to_vec(
                        &hardware_list,
                        bincode::config::standard(),
                    )?);
                }
                _ => {
                    let hardware = hardwares.get_mut(&packet.get_id_num()).unwrap();
                    let payload_ret =
                        hardware.handle_command(packet.get_command(), body.get_payload());
                    if let Ok(payload_ret) = payload_ret {
                        payload = payload_ret;
                    } else {
                        packet.set_error(MsgError::UnsupportedOperation(format!(
                            "Operation not supported by the hardware:{}",
                            packet.get_command()
                        )));
                    }
                }
            }
            send_msg(stream, &MsgBody::new(packet, payload))
                .expect("Failed to send hardware list reply");
            Ok(())
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
                            handle_msg(&mut hardwares, &mut stream, msg).unwrap();
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
