mod constants;
mod plugins;

use bevy::{
    app::{App, Startup},
    prelude::{Camera2dBundle, ClearColor, Commands},
    DefaultPlugins,
};
use constants::BACKGROUND_COLOR;
use plugins::{PaddlePlugin, WallsPlugin};

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
