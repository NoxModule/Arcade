use bevy::prelude::Resource;
use clap::Parser;

/// Welcome to Breakout!
#[derive(Parser, Resource)]
pub struct CliArguments {
    /// Skip splash screens and navigate directly to the main menu.
    #[arg(short, long)]
    pub skip_splash: bool,
}
