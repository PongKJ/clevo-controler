use core::num;
use std::thread;

use clevo_controler::service::core::Service;
use clevo_controler::temp_controler::Controler;
use ksni::Handle;
use std::sync::{Arc, Mutex};

// async fn show_tray_status(tray: &Handle<MyTray>) {
//     // Simulate some work
//     loop {
//         tokio::time::sleep(std::time::Duration::from_secs(5)).await;
//         tray.update(|tray: &mut MyTray| {
//             dbg!(tray);
//         })
//         .await
//         .unwrap();
//     }
// }

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let tray = MyTray::new(0, false);
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
            drop(service);
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });
    handle.join();
    // tray.update(|tray: &mut MyTray| {
    //     tray.checked = false;
    // }).await;
    // tokio::spawn(async move {
    //     // Run some work in the background
    //     show_tray_status(&handle).await
    // });
    // Run forever
    // std::future::pending().await
}
