mod camera_plugin;
mod game_plugin;
mod splash_screen_plugin;
mod states;
mod systems;

use bevy::{
    app::App,
    color::Color,
    prelude::{AppExtStates, ClearColor},
    DefaultPlugins,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            camera_plugin::CameraPlugin,
            game_plugin::GamePlugin,
            splash_screen_plugin::SplashScreenPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb_u8(40, 40, 40)))
        .init_state::<states::GameState>()
        .run();
}
