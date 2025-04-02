use clevo_fancontrold::hardware::cpu::Cpu;
use clevo_fancontrold::service::core::Service;
use lib::proto::*;
use lib::stream::StreamListener;
use std::thread;

fn main() {
    let mut service = Service::new("clevo-fancontrold.sock").expect("Failed to create service");
    let cpu = Cpu::new();
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
