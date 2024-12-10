mod camera_plugin;
mod cli_arguments;
mod game_plugin;
mod main_menu_plugin;
mod splash_screen_plugin;
mod states;
mod systems;
mod user_interface;

use bevy::{
    app::{App, Update},
    prelude::{AppExtStates, EventReader, PluginGroup, Single},
    utils::default,
    window::{Window, WindowPlugin, WindowResized, WindowResolution},
    DefaultPlugins,
};
use clap::Parser;

pub use crate::user_interface::UserInterface;

fn main() {
    App::new()
        .insert_resource(cli_arguments::CliArguments::parse())
        .add_plugins((
            (DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(264.0, 380.0),
                    title: String::from("Arcade"),
                    ..default()
                }),
                ..default()
            })),
            camera_plugin::CameraPlugin,
            game_plugin::GamePlugin,
            main_menu_plugin::MainMenuPlugin,
            splash_screen_plugin::SplashScreenPlugin,
        ))
        .add_systems(Update, on_resize)
        .init_state::<states::GameState>()
        .run();
}

fn on_resize(mut event_reader: EventReader<WindowResized>, mut window: Single<&mut Window>) {
    for _ in event_reader.read() {
        window.resolution.set_scale_factor_override(Some(3.0));
    }
}
