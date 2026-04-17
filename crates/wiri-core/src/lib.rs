use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug)]
#[command(name = "wiri", about = "Wiri window manager CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
pub enum Command {
    Window {
        #[command(subcommand)]
        action: WindowAction,
    },
    Overview {
        #[command(subcommand)]
        action: OverviewAction,
    },
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
pub enum WindowAction {
    Focus { direction: Direction },
}

#[derive(Subcommand, Serialize, Deserialize, Debug)]
pub enum OverviewAction {
    Toggle,
}

#[derive(clap::ValueEnum, Serialize, Deserialize, Clone, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
