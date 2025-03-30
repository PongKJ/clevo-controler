use crate::communicate::reply;
use clevo_fancontrol::communicate::CommuError;
use clevo_fancontrol::communicate::query;
use clevo_fancontrol::communicate::stream::StreamListener;
use std::thread::JoinHandle;

type Result<T> = std::result::Result<T, CommuError>;

pub struct ServiceConfig {
    socket_name: String,
}

pub struct Service {
    config: ServiceConfig,
}

impl Service {
    pub fn new(socket_name: &str) -> Result<Self> {
        Ok(Self {
            config: ServiceConfig {
                socket_name: socket_name.to_string(),
            },
        })
    }

    pub fn spawn(&mut self) -> Result<JoinHandle<()>> {
        let socket_name = self.config.socket_name.clone();
        let handle = std::thread::spawn(move || {
            let mut stream_listener = StreamListener::new(socket_name.as_str()).unwrap();
            loop {
                let mut stream = stream_listener
                    .accept()
                    .expect("Failed to accept stream connection");
                println!("Stream accepted, starting to handle requests...");
                loop {
                    query::reply_query(&mut stream, Box::new(reply::handler)).unwrap();
                }
            }
        });
        Ok(handle)
    }
}
