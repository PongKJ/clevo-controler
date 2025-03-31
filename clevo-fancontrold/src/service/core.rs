use lib::proto::*;
use lib::stream::StreamListener;
use std::thread::JoinHandle;

type Result<T> = std::result::Result<T, ProtoError>;

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
                    let msg_packet = MsgPacket {
                        mode: MsgMode::Request,          // Request mode for incoming messages
                        command: MsgCommand::GetCpuDesc, // Example command to get CPU description
                        error: None,                     // No error for initial request
                        sequence: 555555555555555, // Example sequence number, can be incremented for each request
                    };
                    let payload: Option<String> = Some("CPU description".to_string()); // Example payload
                    send_msg(&mut stream, &msg_packet, &payload).unwrap();
                    match recv_msg(&mut stream) {
                        Ok(msg) => {
                            match msg.packet.command {
                                MsgCommand::GetCpuDesc => {
                                    let payload = None;
                                    send_msg(&mut stream, &msg.packet, &payload)
                                        .expect("Failed to send CPU description reply");
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
