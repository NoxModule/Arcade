mod components;
mod plugins;

use bevy::{
    app::{App, Startup},
    color::Color,
    prelude::{Camera2dBundle, ClearColor, Commands},
    DefaultPlugins,
};
use plugins::{PaddlePlugin, WallsPlugin};

const BACKGROUND_COLOR: Color = Color::srgb(0.392, 0.584, 0.929);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PaddlePlugin, WallsPlugin))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
