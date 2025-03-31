use clevo_fancontrol::domain::hardware::cpu::Cpu;
use clevo_fancontrol::service::core::Service;
use clevo_fancontrol::ui::tray::MyTray;
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
        dotenv::var("SOCKET_NAME").unwrap_or_else(|_| "clevo-fancontrold.sock".to_string());
    let mut service = Service::new(socket_name.as_str()).expect("Failed to create service");
    let cpu = Cpu::new();
    service.add_hardware(Box::new(cpu)).unwrap();
    let handle = service.spawn().unwrap();
    handle.join().unwrap();

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
