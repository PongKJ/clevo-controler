use clevo_fancontrold::service::core::Service;
use std::thread;
fn main() {
    let mut service = Service::new("clevo-fancontrold.sock").expect("Failed to create service");
    let handle = service.spawn().expect("Failed to spawn service");
    handle.join().expect("Service thread has panicked");
}
