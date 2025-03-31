pub mod field;
pub mod proto;
pub mod stream;

#[cfg(test)]
mod tests {
    use super::proto::*;
    use super::stream::*;
    use std::thread;
    use std::time::Duration;

    const NAME: &str = "test_socket";

    #[test]
    fn test_socket_stream() {
        thread::spawn(move || {
            let mut listener = StreamListener::new(NAME).expect("Failed to create listener");
            let mut stream = listener.accept().expect("Failed to accept connection");
            let msg = recv_msg(&mut stream).expect("Failed to receive message");
            dbg!(&msg);
            assert_eq!(msg.header.version, 1);
            assert_eq!(msg.packet.sequence, 1);
            assert!(msg.payload.is_some());
            assert_eq!(
                msg.payload.unwrap(),
                bincode::encode_to_vec("payload from client", bincode::config::standard()).unwrap()
            );
            assert_eq!(msg.packet.command, MsgCommand::GetCpuDesc);
            match msg.packet.command {
                MsgCommand::GetCpuDesc => {
                    // Respond with a predefined message
                    let response = MsgPacket {
                        mode: MsgMode::Response,
                        error: None,
                        sequence: msg.packet.sequence, // Echo back the same sequence
                        command: MsgCommand::GetGpuDesc, // Same command
                    };
                    let payload = Some("payload from server".to_string());
                    send_msg(&mut stream, &response, &payload).expect("Failed to send response");
                }
                _ => {
                    panic!("Received unexpected command: {:?}", msg.packet.command);
                }
            }
        });

        // Give some time for the listener to start
        thread::sleep(Duration::from_millis(50));

        // Connect to the listener
        let mut client_stream = SocketStream::new(NAME).expect("Failed to connect to socket");
        let msg_packet = MsgPacket {
            mode: MsgMode::Request,
            error: None,
            sequence: 1,                     // Example sequence number
            command: MsgCommand::GetCpuDesc, // Request CPU info
        };
        let payload: Option<String> = Some("payload from client".to_string()); // No payload for the request
        send_msg(&mut client_stream, &msg_packet, &payload).expect("Failed to send message");
        let response = recv_msg(&mut client_stream).expect("Failed to receive response");
        assert_eq!(response.header.version, 1);
        assert!(response.payload.is_some());
        assert_eq!(
            response.payload.unwrap(),
            bincode::encode_to_vec("payload from server", bincode::config::standard()).unwrap()
        );
        assert_eq!(response.packet.command, MsgCommand::GetGpuDesc);
        assert_eq!(response.packet.sequence, msg_packet.sequence);
        assert_eq!(response.packet.mode, MsgMode::Response);
        assert!(
            response.packet.error.is_none(),
            "Expected no error in response"
        );
    }
}
