use std::thread;

use clevo_controler::controler::Controler;
use clevo_controler::{service::core::Service, ui::tray::MyTray};
use ksni::Handle;
use std::sync::{Arc, Mutex};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let tray = MyTray::new(0, false);
    let socket_name =
        dotenv::var("SOCKET_NAME").unwrap_or_else(|_| "clevo-controler.sock".to_string());
    let service = Arc::new(Mutex::new(
        Service::new(socket_name.as_str()).expect("Failed to create service"),
    ));
    let service_clone = service.clone();
    thread::spawn(move || {
        let mut contorler = Controler::new(90.0);
        loop {
            let mut service = service_clone.lock().unwrap();
            service.accept(0, &mut contorler);
            drop(service);
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });
    let communicator_handle = service
        .lock()
        .unwrap()
        .spawn_communicator()
        .expect("Failed to spawn receiver thread");
    let refresh_handle = service
        .lock()
        .unwrap()
        .spawn_refresher()
        .expect("Failed to spawn refresher thread");
    communicator_handle.join().unwrap();
    refresh_handle.join().unwrap();
}
