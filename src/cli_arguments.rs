use bevy::prelude::Resource;
use clap::Parser;

/// Welcome to The Arcade!
#[derive(Parser, Resource)]
pub struct CliArguments {
    /// Skip splash screens and navigate directly to the main menu.
    #[arg(short, long)]
    pub skip_splash: bool,
}
