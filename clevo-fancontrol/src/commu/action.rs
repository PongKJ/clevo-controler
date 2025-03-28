use super::CommuError;
use crate::domain::hw::HwInfo;
use serde::{Deserialize, Serialize};

use super::localstream::LocalStream;
use crate::domain::hw::field::hwname::HwName;
use crate::domain::hw::parameter::SetDeviceParam;

type Result<T> = std::result::Result<T, CommuError>;

#[derive(Debug, Serialize, Deserialize)]
pub enum MsgType {
    ScanDevice,
    SetDevice(HwName, SetDeviceParam),
    GetDeviceInfo(HwName),
}

pub fn scan_devices(stream: &mut LocalStream) -> Result<Vec<HwName>> {
    let mut devices = vec![];
    let msg = serde_json::to_string(&MsgType::ScanDevice)?;
    stream.write(msg.as_bytes())?;
    let response = stream.read()?;
    if let Ok(devices_list) = serde_json::from_str::<Vec<HwName>>(&response) {
        devices = devices_list;
    } else {
        eprintln!("Failed to parse response: {}", response);
    }
    Ok(devices)
}

pub fn get_device_info(stream: &mut LocalStream, hw_name: HwName) -> Result<HwInfo> {
    let msg = serde_json::to_string(&MsgType::GetDeviceInfo(hw_name))?;
    stream.write(msg.as_bytes())?;
    let response = stream.read()?;
    if let Ok(hw_info) = serde_json::from_str::<HwInfo>(&response) {
        Ok(hw_info)
    } else {
        Err(CommuError::Parse(response))
    }
}

pub fn set_device(
    stream: &mut LocalStream,
    hw_name: HwName,
    param: SetDeviceParam,
) -> Result<HwInfo> {
    let msg = serde_json::to_string(&MsgType::SetDevice(hw_name, param))?;
    stream.write(msg.as_bytes())?;
    let response = stream.read()?;
    if let Ok(hw_info) = serde_json::from_str::<HwInfo>(&response) {
        Ok(hw_info)
    } else {
        Err(CommuError::Parse(response))
    }
}
