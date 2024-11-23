mod components;
mod plugins;
mod states;
mod systems;

use bevy::{
    app::{App, FixedUpdate},
    color::Color,
    prelude::{in_state, AppExtStates, ClearColor, IntoSystemConfigs},
    DefaultPlugins,
};

use crate::{
    plugins::{
        BallPlugin, BrickPlugin, CameraPlugin, ColliderPlugin, PaddlePlugin, SplashScreenPlugin,
        WallsPlugin,
    },
    states::GameState,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BallPlugin,
            BrickPlugin,
            CameraPlugin,
            ColliderPlugin,
            PaddlePlugin,
            SplashScreenPlugin,
            WallsPlugin,
        ))
        .add_systems(
            FixedUpdate,
            (
                BallPlugin::apply_velocity.run_if(in_state(GameState::InGame)),
                PaddlePlugin::move_paddle.run_if(in_state(GameState::InGame)),
                ColliderPlugin::check_collisions.run_if(in_state(GameState::InGame)),
            )
                .chain(),
        )
        .insert_resource(ClearColor(Color::srgb_u8(40, 40, 40)))
        .init_state::<GameState>()
        .run();
}
