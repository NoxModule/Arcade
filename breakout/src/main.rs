mod components;
mod plugins;

use bevy::{
    app::{App, FixedUpdate},
    color::Color,
    prelude::{ClearColor, IntoSystemConfigs},
    DefaultPlugins,
};

use crate::plugins::{
    BallPlugin, BrickPlugin, CameraPlugin, ColliderPlugin, PaddlePlugin, WallsPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            BallPlugin,
            BrickPlugin,
            CameraPlugin,
            ColliderPlugin,
            PaddlePlugin,
            WallsPlugin,
        ))
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
