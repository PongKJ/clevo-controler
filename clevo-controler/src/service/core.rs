use crate::domain::component::Component;
use crate::domain::component::cpu::Cpu;
use lib::field::ComponentList;
use lib::field::CpuStatus;
use lib::field::desc::{ComponentType, Desc};
use lib::proto::Msg;
use lib::proto::MsgBody;
use lib::proto::MsgCommand;
use lib::proto::MsgMode;
use lib::proto::MsgPacket;
use lib::proto::ProtoError;
use lib::proto::recv_msg;
use lib::proto::send_msg;
use lib::stream::SocketStream;

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

type Result<T> = std::result::Result<T, ProtoError>;

pub struct ServiceConfig {
    interval: u64, // Time interval for refreshing hardware data by seconds
}

pub struct Service {
    config: ServiceConfig,
    components: Arc<Mutex<HashMap<u8, Box<dyn Component + Send + Sync>>>>,
    socket_stream: Arc<Mutex<SocketStream>>,
    receiver: Arc<Mutex<Receiver<MsgBody>>>,
    sender: Arc<Mutex<Sender<MsgBody>>>,
}

impl Service {
    pub fn new(socket_name: &str) -> Result<Self> {
        let socket_stream = SocketStream::new(socket_name)?;
        let (sender, receiver) = std::sync::mpsc::channel();
        Ok(Self {
            receiver: Arc::new(Mutex::new(receiver)),
            sender: Arc::new(Mutex::new(sender)),
            config: ServiceConfig {
                interval: 5, // Default refresh interval is 5 seconds
            },
            components: Arc::new(Mutex::new(HashMap::new())),
            socket_stream: Arc::new(Mutex::new(socket_stream)),
        })
    }
    fn add_hardware(&mut self, component_list: &ComponentList) {
        let mut components = self.components.lock().unwrap();
        component_list
            .0
            .iter()
            .for_each(|(id_num, desc)| match desc.get_type() {
                ComponentType::Cpu => {
                    let cpu = Cpu::new(*id_num, Arc::clone(&self.sender));
                    components.insert(*id_num, Box::new(cpu));
                }
                ComponentType::Gpu => {
                    todo!()
                }
            });
    }

    // INFO:
    // 1. Service call components to refresh their status
    // 2. components return a Msg
    // 3. Service send the Msg to the socket stream
    // 4. Service receive the reply from the socket stream
    // 5. Service divide the reply by index and dispach them to the sepecific components
    // 服务端首先会检测生成硬件列表，客户端通过 GetComponentList 来获取硬件列表,
    // 根据得到的硬件类型创建对应的硬件对象，添加到components中

    // TODO: add index field

    pub fn spawn_communicator(&mut self) -> Result<JoinHandle<()>> {
        // get component list first
        let packet = MsgPacket::new(MsgMode::Reply, None, 0, 0, MsgCommand::GetComponentList)
            .serialize()
            .expect("Failed to serialize MsgPacket");
        let body = MsgBody::new(packet.clone(), None);
        let mut socket_stream = self.socket_stream.lock().unwrap();
        send_msg(&mut socket_stream, &body).expect("Failed to send message");
        let reply_msg = recv_msg(&mut socket_stream).unwrap();
        drop(socket_stream); // need to drop the lock before spawning a new thread
        let (component_list, _): (ComponentList, _) = bincode::decode_from_slice(
            reply_msg.get_payload().as_deref().unwrap_or(&[]),
            bincode::config::standard(),
        )
        .expect("Failed to decode hardware list");
        dbg!(&component_list);
        self.add_hardware(&component_list);

        // messsage reveiwer thread start
        let components_clone = Arc::clone(&self.components);
        let socket_stream_clone = Arc::clone(&self.socket_stream);
        let receiver_clone = Arc::clone(&self.receiver);

        let handle = std::thread::spawn(move || {
            loop {
                let mut socket_stream = socket_stream_clone.lock().unwrap();
                let receiver = receiver_clone.lock().unwrap();
                match receiver.recv() {
                    Ok(body) => {
                        send_msg(&mut socket_stream, &body).unwrap();
                    }
                    Err(e) => {
                        panic!("Failed to receive message: {}", e);
                    }
                }
                let body = recv_msg(&mut socket_stream).expect("Failed to receive message");
                let packet = MsgPacket::deserialize(body.get_packet()).unwrap();
                println!("Received command: {:?}", packet.get_command());
                let mut components = components_clone.lock().unwrap();
                if let Some(component) = components.get_mut(&packet.get_id_num()) {
                    component
                        .refresh_from_reply(packet.get_command(), body.get_payload())
                        .unwrap();
                } else {
                    eprintln!("Component not found for index: {}", packet.get_id_num());
                }
                drop(socket_stream);
            }
        });
        Ok(handle)
    }

    pub fn spawn_refresher(&mut self) -> Result<JoinHandle<()>> {
        // refresh component status
        let components_clone = Arc::clone(&self.components);
        let sender_clone = Arc::clone(&self.sender);
        let interval = self.config.interval;

        let handle = std::thread::spawn(move || {
            loop {
                let components = components_clone.lock().unwrap();
                components.iter().for_each(|(index, hardware)| {
                    let packet =
                        MsgPacket::new(MsgMode::Reply, None, 0, *index, MsgCommand::GetStatus)
                            .serialize()
                            .expect("Failed to serialize MsgPacket");
                    let body = MsgBody::new(packet.clone(), None);
                    let sender = sender_clone.lock().unwrap();
                    // hardware.refresh_status().unwrap();
                    sender.send(body).unwrap();
                });
                drop(components);
                std::thread::sleep(std::time::Duration::from_secs(interval));
            }
        });
        Ok(handle)
    }

    // TODO: Some problem in structure design, Can't expose components interface outside  
    pub fn get_components(&self) -> Arc<Mutex<HashMap<u8, Box<dyn Component + Send + Sync>>>> {
        Arc::clone(&self.components)
    }
}
