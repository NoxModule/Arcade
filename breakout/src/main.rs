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

const BACKGROUND_COLOR: Color = Color::srgb(0.392, 0.584, 0.929);

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
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
