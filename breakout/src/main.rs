mod constants;

use bevy::{
    app::{App, Startup},
    prelude::{Camera2dBundle, ClearColor, Commands},
    DefaultPlugins,
};

use crate::constants::BACKGROUND_COLOR;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
