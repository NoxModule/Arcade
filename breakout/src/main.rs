mod components;
mod plugins;

use bevy::{
    app::{App, FixedUpdate, Startup},
    color::Color,
    prelude::{Camera2dBundle, ClearColor, Commands, IntoSystemConfigs},
    DefaultPlugins,
};
use plugins::{BallPlugin, ColliderPlugin, PaddlePlugin, WallsPlugin};

const BACKGROUND_COLOR: Color = Color::srgb(0.392, 0.584, 0.929);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BallPlugin,
            ColliderPlugin,
            PaddlePlugin,
            WallsPlugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                BallPlugin::apply_velocity,
                PaddlePlugin::move_paddle,
                ColliderPlugin::check_collisions,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
