use clevo_controlerd::component::cpu::Cpu;
use clevo_controlerd::lowlevel::accessor::fd::Fd;
// use clevo_controlerd::{hardware::cpu::Cpu, service::core::Service};
use lib::proto::*;
use lib::stream::StreamListener;
use std::thread;

use clevo_controlerd::component::gpu::Gpu;
use clevo_controlerd::service::core::Service;

fn main() {
    // let mut gpu = Gpu::init().expect("Failed to initialize GPU");
    // gpu.refresh().expect("Failed to refresh GPU");

    let mut service = Service::new("clevo-controler.sock").expect("Failed to create service");
    let cpu = Cpu::init().unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    service
        .add_hardware(0, Box::new(cpu))
        .expect("Failed to add hardware");
    let monitor_handle = service.spawn_monitor().expect("Failed to spawn service");
    let msg_handler_handle = service
        .spawn_msg_handler()
        .expect("Failed to spawn service");
    msg_handler_handle
        .join()
        .expect("Service thread has panicked");
    monitor_handle.join().expect("Service thread has panicked");
}
