use clevo_controlerd::component::cpu::intel::IntelCpu;
use clevo_controlerd::component::fan::Fan;
use clevo_controlerd::service::core::Service;

fn main() {
    let mut service = Service::new("clevo-controler.sock").expect("Failed to create service");
    let cpu = IntelCpu::init(0).unwrap();
    let fan = Fan::new();
    std::thread::sleep(std::time::Duration::from_secs(1));
    service
        .add_hardware(0, Box::new(cpu))
        .expect("Failed to add hardware");
    service
        .add_hardware(1, Box::new(fan))
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
