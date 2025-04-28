use clevo_controller::service::core::Service;
use clevo_controller::temp_controler::Controler;
use std::env;
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
        let config_path = format!(
            "{}{}",
            env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap(),
            "/configs.json"
        );
        dbg!(&config_path);
        let mut contorler = Controler::new(&config_path);
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
