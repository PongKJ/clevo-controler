use crate::domain::field::{fan_speed::TargetFanSpeed, temp::Temp};
use std::collections::HashMap;

pub struct ControlByTable {
    tabel: HashMap<Temp, TargetFanSpeed>, // Map from target temperature to target fan speed
}
