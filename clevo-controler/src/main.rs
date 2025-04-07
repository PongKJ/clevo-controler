use clevo_controler::{domain::component::cpu::Cpu, service::core::Service, ui::tray::MyTray};
use ksni::Handle;

async fn show_tray_status(tray: &Handle<MyTray>) {
    // Simulate some work
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        tray.update(|tray: &mut MyTray| {
            dbg!(tray);
        })
        .await
        .unwrap();
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // let tray = MyTray::new(0, false);
    let socket_name =
        dotenv::var("SOCKET_NAME").unwrap_or_else(|_| "clevo-controler.sock".to_string());
    let mut service = Service::new(socket_name.as_str()).expect("Failed to create service");
    let communicator_handle = service
        .spawn_communicator()
        .expect("Failed to spawn receiver thread");
    let refresh_handle = service
        .spawn_refresher()
        .expect("Failed to spawn refresher thread");
    communicator_handle.join().unwrap();
    refresh_handle.join().unwrap();
    let components = service.get_components();
    // components
    //     .lock()
    //     .unwrap()
    //     .iter()
    //     .for_each(|(index, hardware)| {
    //         println!("Component {}: {:?}", index, hardware.get_desc());
    //     });

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
