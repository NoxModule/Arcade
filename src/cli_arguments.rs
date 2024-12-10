use bevy::prelude::Resource;
use clap::Parser;

/// Welcome to The Arcade!
#[derive(Parser, Resource)]
pub struct CliArguments {
    /// Skip the main menu and navigate directly to to the game.
    #[arg(long)]
    pub skip_menu: bool,

    /// Skip splash screens and navigate directly to the main menu.
    #[arg(long)]
    pub skip_splash: bool,
}
