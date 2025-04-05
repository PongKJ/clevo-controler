pub mod proto;
pub mod query;
pub mod stream;

#[derive(Debug)]
pub enum CommuError {
    Stream(String),
    Parse(String),
    InvalidReply(String),
}

impl From<serde_json::Error> for CommuError {
    fn from(err: serde_json::Error) -> Self {
        CommuError::Parse(err.to_string())
    }
}

impl From<stream::StreamError> for CommuError {
    fn from(err: stream::StreamError) -> Self {
        CommuError::Stream(err.to_string())
    }
}

impl From<CommuError> for String {
    fn from(value: CommuError) -> Self {
        match value {
            CommuError::Stream(msg) => format!("Stream error: {}", msg),
            CommuError::Parse(msg) => format!("Parse error: {}", msg),
            CommuError::InvalidReply(msg) => format!("Invalid reply: {}", msg),
        }
    }
}

// #[cfg(test)]
// pub mod test {
//     use super::*;
//     use crate::domain::hardware::parameter::SetDeviceParam;
//     use crate::domain::hardware::{HwInfo, field::*};
//     use query::MsgType;
//     use fan::{Fan, FanMode};
//     use indentifier::{DeviceType, HwName};
//     use std::thread;
//     use std::time::Duration;
//     use usage::HwUsage;
//
//     const NAME: &str = "clevo-fancontrold.sock";
//     fn handle_request(request: String) -> String {
//         let msg = serde_json::from_str::<query::MsgType>(&request).expect("invalid request:{}");
//         let fake_devices = vec![indentifier::HwName::new(
//             indentifier::DeviceType::CPU,
//             "fake cpu".into(),
//         )];
//
//         let fake_hw_info = HwInfo::new(HwUsage::new(3.0, 1.0, 60.0, 20.0, 1.1, 1.0));
//
//         match msg {
//             MsgType::ScanDevice => serde_json::to_string(&fake_devices).unwrap(),
//             MsgType::GetDeviceInfo(hwname) => {
//                 assert_eq!(hwname.get_device_type(), indentifier::DeviceType::CPU);
//                 serde_json::to_string(&fake_hw_info).unwrap()
//             }
//             MsgType::SetDevice(hw_name, param) => {
//                 assert_eq!(hw_name.get_device_type(), indentifier::DeviceType::CPU);
//                 assert_eq!(hw_name.get_desc(), "fake cpu");
//                 assert_eq!(param.fan.rpm, 3500);
//                 assert_eq!(param.fan.duty, 0.6);
//                 serde_json::to_string(&fake_hw_info).unwrap()
//             }
//         }
//     }
//     fn run_listener() {
//         use {
//             interprocess::local_socket::{GenericNamespaced, ListenerOptions, Stream, prelude::*},
//             std::io::{self, BufReader, prelude::*},
//         };
//
//         fn handle_error(conn: io::Result<Stream>) -> Option<Stream> {
//             match conn {
//                 Ok(c) => Some(c),
//                 Err(e) => {
//                     eprintln!("Incoming connection failed: {e}");
//                     None
//                 }
//             }
//         }
//         let name = NAME.to_ns_name::<GenericNamespaced>().unwrap();
//         let opts = ListenerOptions::new().name(name);
//         let listener = opts.create_sync().unwrap();
//         let mut buffer = vec![];
//
//         let conn = listener.accept().unwrap();
//         let mut conn = BufReader::new(conn);
//         loop {
//             conn.read_until(b'-', &mut buffer).expect("failed to read");
//             buffer.truncate(buffer.len() - 1);
//             let msg_response = handle_request(String::from_utf8(buffer.clone()).unwrap());
//             conn.get_mut()
//                 .write_all(msg_response.as_bytes())
//                 .expect("local stream write failed");
//             // Use '-' as a terminator
//             conn.get_mut()
//                 .write_all(b"-")
//                 .expect("local stream write failed");
//             buffer.clear();
//         }
//     }
//
//     #[test]
//     fn test_commu() {
//         thread::spawn(move || {
//             run_listener();
//         });
//         thread::sleep(Duration::from_millis(50));
//         let mut local_stream = stream::LocalStream::new(NAME.to_string()).unwrap();
//         query::scan_devices(&mut local_stream).expect("failed to scan devices");
//         query::get_device_info(
//             &mut local_stream,
//             indentifier::HwName::new(DeviceType::CPU, "fake cpu".into()),
//         )
//         .expect("failed to get device info");
//         query::set_device(
//             &mut local_stream,
//             HwName::new(DeviceType::CPU, "fake cpu".into()),
//             SetDeviceParam::new(Fan {
//                 mode: FanMode::Auto,
//                 rpm: 3500,
//                 duty: 0.6,
//             }),
//         )
//         .expect("faild to set device");
//     }
// }
