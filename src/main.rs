mod camera_plugin;
mod cli_arguments;
mod game_plugin;
mod main_menu_plugin;
mod splash_screen_plugin;
mod states;
mod systems;
mod user_interface;

use bevy::{
    app::App,
    color::Color,
    prelude::{AppExtStates, ClearColor},
    DefaultPlugins,
};
use clap::Parser;

pub use crate::user_interface::UserInterface;

fn main() {
    App::new()
        .insert_resource(cli_arguments::CliArguments::parse())
        .add_plugins((
            DefaultPlugins,
            camera_plugin::CameraPlugin,
            game_plugin::GamePlugin,
            main_menu_plugin::MainMenuPlugin,
            splash_screen_plugin::SplashScreenPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb_u8(40, 40, 40)))
        .init_state::<states::GameState>()
        .run();
}
