use clap::Parser;
use std::fs::OpenOptions;
use std::io::Write;
use wiri_core::Cli;

fn main() {
    let args = Cli::parse();
    let json_payload = serde_json::to_string(&args.command).unwrap();

    let pipe_name = r"\\.\pipe\wiri-ipc";

    let mut pipe = OpenOptions::new()
        .write(true)
        .open(pipe_name)
        .unwrap_or_else(|_| {
            eprintln!(
                "Failed to connect to Wiri daemon. Please make sure it is running in the background"
            );
            std::process::exit(1);
        });

    writeln!(pipe, "{}", json_payload).unwrap();
}
