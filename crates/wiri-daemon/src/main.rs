use std::sync::mpsc;
use std::thread;
use tokio::io::AsyncReadExt;
use tokio::net::windows::named_pipe::ServerOptions;
use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};
use wiri_core::Command;

fn main() {
    // 1. Create a channel to send commands from the Tokio thread to the Main thread
    let (tx, rx) = mpsc::channel::<Command>();

    // 2. Spawn the Tokio runtime in a background thread
    thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            run_ipc_server(tx).await;
        });
    });

    println!("Wiri Daemon started.");
    println!("Waiting for commands on main thread...");

    // 3. The Main Win32 Event Loop (Must be on the main thread)
    // We use a non-blocking check for the channel, and a blocking GetMessage for Win32
    loop {
        // Check if the CLI sent us a command
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

        // Wait for CLI to connect
        server.connect().await.unwrap();

        let mut buffer = String::new();
        // Read until the CLI closes the connection
        server.read_to_string(&mut buffer).await.unwrap();

        if let Ok(command) = serde_json::from_str::<Command>(&buffer.trim()) {
            // Send the command to the Main Win32 Thread
            if tx.send(command).is_err() {
                break; // Main thread closed
            }
        }

        // Loop restarts, re-creating the pipe for the next command
    }
}
