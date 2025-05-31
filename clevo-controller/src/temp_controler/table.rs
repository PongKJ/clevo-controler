use lib::field::{fan_speed::TargetFanSpeed, temp::Temp};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct ControlByTable {
    tabel: HashMap<Temp, TargetFanSpeed>, // Map from target temperature to target fan speed
}
