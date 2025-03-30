use crate::communicate::CommuError;
use crate::communicate::stream::StreamClient;
use crate::domain::hardware::Hardware;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread::JoinHandle;

type Result<T> = std::result::Result<T, CommuError>;

pub struct ServiceConfig {
    socket_name: String,
    interval: u64, // Time interval for refreshing hardware data by seconds
}

pub struct Service {
    config: ServiceConfig,
    hardwares: Arc<Mutex<Vec<Box<dyn Hardware + Send + Sync>>>>,
}

impl Service {
    pub fn new(socket_name: &str) -> Result<Self> {
        Ok(Self {
            config: ServiceConfig {
                socket_name: socket_name.to_string(),
                interval: 5, // Default refresh interval is 5 seconds
            },
            hardwares: Arc::new(Mutex::new(vec![])),
        })
    }
    pub fn print_hardware_status(&self) {
        let hardwares = self.hardwares.lock().unwrap();
        for hardware in hardwares.iter() {
            println!("Hardware Description: {}", hardware.get_desc());
            if let Ok(freq) = hardware.get_freq() {
                println!("Frequency: {:?}", freq);
            }
            if let Ok(usage) = hardware.get_usage() {
                println!("Usage: {:?}", usage);
            }
            if let Ok(temp) = hardware.get_temp() {
                println!("Temperature: {:?}", temp);
            }
            if let Ok(power) = hardware.get_power() {
                println!("Power: {:?}", power);
            }
            if let Ok(fan_speed) = hardware.get_fan_speed() {
                println!("Fan Speed: {:?}", fan_speed);
            }
        }
        println!("-----------------------------------");
    }
    pub fn add_hardware(&mut self, hardware: Box<dyn Hardware + Send + Sync>) -> Result<()> {
        let mut hardwares = self.hardwares.lock().unwrap();
        hardwares.push(hardware);
        Ok(())
    }

    pub fn spawn(&mut self) -> Result<JoinHandle<()>> {
        let hardwares_clone = Arc::clone(&self.hardwares);
        let interval = self.config.interval;
        let socket_name = self.config.socket_name.clone();
        let handle = std::thread::spawn(move || {
            let mut stream_client = StreamClient::new(socket_name.as_str()).unwrap();
            loop {
                let mut hardwares = hardwares_clone.lock().unwrap();
                hardwares.iter_mut().for_each(|hardware| {
                    if let Err(e) = hardware.refresh(&mut stream_client) {
                        eprintln!("Failed to refresh hardware: {:?}", e);
                    }
                });
                drop(hardwares);
                std::thread::sleep(std::time::Duration::from_secs(interval));
            }
        });
        Ok(handle)
    }
}
