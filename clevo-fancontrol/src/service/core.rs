use crate::domain::hardware::Hardware;
use lib::proto::ProtoError;
use lib::proto::recv_msg;
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
            loop {
                let mut socket_stream_clone = socket_stream_clone.lock().unwrap();
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
