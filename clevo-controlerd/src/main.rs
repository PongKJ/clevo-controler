use clevo_controlerd::lowlevel::accessor::fd::Fd;
use clevo_controlerd::lowlevel::accessor::intel_rpal;
use clevo_controlerd::{hardware::cpu::Cpu, service::core::Service};
use lib::proto::*;
use lib::stream::StreamListener;
use std::thread;

fn main() {
    let mut msr = intel_rpal::RpalAccessor::new();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let power = msr.get_period_power().expect("Failed to get period power");
    dbg!(power);
    // let mut service = Service::new("clevo-controler.sock").expect("Failed to create service");
    // let cpu = Cpu::new();
    // service
    //     .add_hardware(0, Box::new(cpu))
    //     .expect("Failed to add hardware");
    // let monitor_handle = service.spawn_monitor().expect("Failed to spawn service");
    // let msg_handler_handle = service
    //     .spawn_msg_handler()
    //     .expect("Failed to spawn service");
    // msg_handler_handle
    //     .join()
    //     .expect("Service thread has panicked");
    // monitor_handle.join().expect("Service thread has panicked");
}
