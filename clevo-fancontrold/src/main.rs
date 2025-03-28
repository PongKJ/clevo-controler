use std::thread;

use clevo_fancontrold::hardware::peripheries::fan::{self, FanIndex};
fn main() {
    let fan = fan::FanCtler::new();
    // fan.set_fan_speed(0.6, FanIndex::CPU);
    fan.set_fan_auto(FanIndex::GPU);
    loop {
        let rpm = fan.get_fan_rpm(FanIndex::GPU);
        dbg!(rpm);
        thread::sleep(std::time::Duration::from_secs(1));
    }
}
