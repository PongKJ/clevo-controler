use clevo_fancontrol::commu::localstream::LocalStream;
use clevo_fancontrol::ui::tray::MyTray;
use interprocess::{
    TryClone,
    local_socket::{GenericFilePath, GenericNamespaced, Stream, prelude::*},
};
use ksni::{Handle, TrayMethods};
use std::io::{BufRead, BufReader, BufWriter, Write};

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
    let tray = MyTray::new(0, false);
    let handle = tray.spawn().await.unwrap();
    let mut local_stream = LocalStream::new("clevo-fancontrold.sock".to_string()).unwrap();
    local_stream.write(b"Hello\n").unwrap();
    let msg = local_stream.read().unwrap();
    dbg!(msg);

    // tray.update(|tray: &mut MyTray| {
    //     tray.checked = false;
    // }).await;
    // tokio::spawn(async move {
    //     // Run some work in the background
    //     show_tray_status(&handle).await
    // });
    // Run forever
    std::future::pending().await
}
