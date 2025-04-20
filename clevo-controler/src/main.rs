use clevo_controler::service::core::Service;
use clevo_controler::temp_controler::Controler;
use std::sync::{Arc, Mutex};
use std::thread;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let socket_name =
        dotenv::var("SOCKET_NAME").unwrap_or_else(|_| "clevo-controler.sock".to_string());
    let (service, handle) = Service::init(socket_name.as_str()).expect("Failed to create service");
    let service = Arc::new(Mutex::new(service));
    let service_clone = service.clone();
    thread::spawn(move || {
        let mut contorler = Controler::new("config.json");
        loop {
            let mut service = service_clone.lock().unwrap();
            service.accept(0, &mut contorler);
            service.accept(1, &mut contorler);
            drop(service);
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    });
    handle.join();
}
