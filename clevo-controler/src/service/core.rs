use crate::component::Component;
use crate::component::Visitor;
use crate::component::cpu::Cpu;
use lib::field::ComponentList;
use lib::field::desc::{ComponentType, Desc};
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

pub struct ComponentInfo {
    desc: Desc,
    active: bool,
}

impl ComponentInfo {
    pub fn new(desc: Desc) -> Self {
        Self { desc, active: true }
    }
    pub fn get_desc(&self) -> &Desc {
        &self.desc
    }
    pub fn is_active(&self) -> bool {
        self.active
    }
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
    }
}

pub struct ServiceConfig {
    interval: u64, // Time interval for refreshing hardware data by seconds
    socket_name: String,
}

pub struct Service {
    config: ServiceConfig,
    components_info: Arc<Mutex<HashMap<u8, ComponentInfo>>>,
    components: Arc<Mutex<HashMap<u8, Box<dyn Component + Send + Sync>>>>,
    sender: Arc<Mutex<Sender<MsgBody>>>,
}

pub struct ServiceHandle {
    communicator_handle: JoinHandle<()>,
    refresher_handle: JoinHandle<()>,
}

impl ServiceHandle {
    pub fn new(communicator_handle: JoinHandle<()>, refresher_handle: JoinHandle<()>) -> Self {
        Self {
            communicator_handle,
            refresher_handle,
        }
    }
    pub fn join(self) {
        self.communicator_handle.join().unwrap();
        self.refresher_handle.join().unwrap();
    }
}

impl Service {
    pub fn init(socket_name: &str) -> Result<(Service, ServiceHandle)> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut service = Self {
            sender: Arc::new(Mutex::new(sender)),
            config: ServiceConfig {
                interval: 5, // Default refresh interval is 5 seconds
                socket_name: socket_name.to_string(),
            },
            components_info: Arc::new(Mutex::new(HashMap::new())),
            components: Arc::new(Mutex::new(HashMap::new())),
        };
        let communicator_handle = service.spawn_communicator(receiver)?;
        let refresher_handle = service.spawn_refresher()?;
        Ok((
            service,
            ServiceHandle {
                communicator_handle,
                refresher_handle,
            },
        ))
    }

    pub fn accept(&mut self, id: u8, visitor: &mut dyn Visitor) {
        let mut components = self.components.lock().unwrap();
        components
            .get_mut(&id)
            .expect("Component not found")
            .accept(visitor);
    }

    pub fn get_components(&self) -> HashMap<u8, Desc> {
        let components_info = self.components_info.lock().unwrap();
        components_info
            .iter()
            .map(|(id, info)| {
                let desc = info.get_desc();
                (*id, desc.clone())
            })
            .collect()
    }
    pub fn active_component(&mut self, id: u8) {
        let mut components_info = self.components_info.lock().unwrap();
        if let Some(component) = components_info.get_mut(&id) {
            component.set_active(true);
        } else {
            eprintln!("Component not found for index: {}", id);
        }
    }
    pub fn deactive_component(&mut self, id: u8) {
        let mut components_info = self.components_info.lock().unwrap();
        if let Some(component) = components_info.get_mut(&id) {
            component.set_active(false);
        } else {
            eprintln!("Component not found for index: {}", id);
        }
    }

    fn add_component(&mut self, component_list: &ComponentList) {
        let mut components = self.components.lock().unwrap();
        let mut components_info = self.components_info.lock().unwrap();
        component_list.0.iter().for_each(|(id_num, desc)| {
            components_info.insert(*id_num, ComponentInfo::new(desc.clone()));
            match desc.get_type() {
                ComponentType::Cpu => {
                    let cpu = Cpu::new(*id_num, Arc::clone(&self.sender));
                    components.insert(*id_num, Box::new(cpu));
                }
                ComponentType::Gpu => {
                    todo!()
                }
            }
        });
    }

    // Get harware list --> create hardware object --> add to components --> msg loop
    fn spawn_communicator(&mut self, receiver: Receiver<MsgBody>) -> Result<JoinHandle<()>> {
        // get component list first
        let packet = MsgPacket::new(MsgMode::Reply, None, 0, 0, MsgCommand::GetComponentList)
            .serialize()
            .expect("Failed to serialize MsgPacket");
        let body = MsgBody::new(packet.clone(), None);
        let mut socket_stream = SocketStream::new(self.config.socket_name.as_str())
            .expect("Failed to create socket stream");
        send_msg(&mut socket_stream, &body).expect("Failed to send message");
        let reply_msg = recv_msg(&mut socket_stream).unwrap();
        let (component_list, _): (ComponentList, _) = bincode::decode_from_slice(
            reply_msg.get_payload().as_deref().unwrap_or(&[]),
            bincode::config::standard(),
        )
        .expect("Failed to decode hardware list");
        dbg!(&component_list);
        self.add_component(&component_list);

        // messsage reveiwer thread start
        let components_clone = Arc::clone(&self.components);

        let handle = std::thread::spawn(move || {
            loop {
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
            }
        });
        Ok(handle)
    }

    fn spawn_refresher(&mut self) -> Result<JoinHandle<()>> {
        // refresh component status
        let components_info_clone = Arc::clone(&self.components_info);
        let sender_clone = Arc::clone(&self.sender);
        let interval = self.config.interval;

        let handle = std::thread::spawn(move || {
            loop {
                let components_info = components_info_clone.lock().unwrap();
                components_info
                    .iter()
                    .filter(|(_, info)| info.is_active())
                    .for_each(|(index, _)| {
                        let packet =
                            MsgPacket::new(MsgMode::Reply, None, 0, *index, MsgCommand::GetStatus)
                                .serialize()
                                .expect("Failed to serialize MsgPacket");
                        let body = MsgBody::new(packet.clone(), None);
                        let sender = sender_clone.lock().unwrap();
                        // hardware.refresh_status().unwrap();
                        sender.send(body).unwrap();
                    });
                drop(components_info);
                std::thread::sleep(std::time::Duration::from_secs(interval));
            }
        });
        Ok(handle)
    }
}
