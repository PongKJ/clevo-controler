use clevo_fancontrol::commu::action::{self, MsgType};
use clevo_fancontrol::domain::hw::parameter;
use clevo_fancontrol::domain::hw::{
    HwInfo,
    field::{
        fan::Fan,
        hwname::{DeviceType, HwName},
        usage::HwUsage,
    },
    parameter::SetDeviceParam,
};

pub trait Handler {
    fn get_device_info() -> HwInfo;
    fn set_deice(param: parameter::SetDeviceParam) -> HwInfo;
}

// fn handle_request(request: String) -> String {
//     let msg = serde_json::from_str::<action::MsgType>(&request).expect("invalid request:{}");
//     let fake_devices = vec![hwname::HwName::new(
//         hwname::DeviceType::CPU,
//         "fake cpu".into(),
//     )];
//
//     let fake_hw_info = HwInfo::new(
//         hwname::HwName::new(hwname::DeviceType::CPU, "fake cpu".into()),
//         HwUsage::new(3.0, 1.0, 60.0, 20.0, 1.1, 1.0),
//         Fan::new(3500, 0.6),
//     );
//
//     match msg {
//         MsgType::ScanDevice => serde_json::to_string(&fake_devices).unwrap(),
//         MsgType::GetDeviceInfo(hwname) => {
//             assert_eq!(hwname.get_device_type(), hwname::DeviceType::CPU);
//             serde_json::to_string(&fake_hw_info).unwrap()
//         }
//         MsgType::SetDevice(hw_name, param) => {
//             assert_eq!(hw_name.get_device_type(), hwname::DeviceType::CPU);
//             assert_eq!(hw_name.get_desc(), "fake cpu");
//             assert_eq!(param.fan().get_speed(), 3500);
//             assert_eq!(param.fan().get_duty(), 0.6);
//             serde_json::to_string(&fake_hw_info).unwrap()
//         }
//     }
// }
