use std::sync::mpsc;
use std::thread;
use tokio::io::AsyncReadExt;
use tokio::net::windows::named_pipe::ServerOptions;
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};
use wiri_core::Command;

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            run_ipc_server(tx).await;
        });
    });

    println!("Wiri Daemon started.");
    println!("Waiting for commands on main thread...");

    loop {
        if let Ok(cmd) = rx.try_recv() {
            println!("Main Thread executing command: {:?}", cmd);
            // DO YOUR WINDOW MANAGEMENT HERE
        }

        // --- WIN32 MESSAGE PUMP PLACEHOLDER ---
        // When you start using SetWinEventHook, Windows sends messages here.
        // For now, to keep the CPU usage at 0% while we wait, we sleep briefly.
        // (Later, GetMessageW will pause the thread automatically).
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

async fn run_ipc_server(tx: mpsc::Sender<Command>) {
    let pipe_name = r"\\.\pipe\wiri-ipc";

    loop {
        let mut server = ServerOptions::new()
            .first_pipe_instance(true)
            .create(pipe_name)
            .expect("Failed to create named pipe server");

        server.connect().await.unwrap();

        let mut buffer = String::new();

        server.read_to_string(&mut buffer).await.unwrap();

        if let Ok(command) = serde_json::from_str::<Command>(&buffer.trim()) {
            if tx.send(command).is_err() {
                break;
            }
        }
    }
}
