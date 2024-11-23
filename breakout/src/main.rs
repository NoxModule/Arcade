mod components;
mod plugins;

use bevy::{
    app::{App, FixedUpdate},
    color::Color,
    prelude::{
        in_state, AppExtStates, ClearColor, Commands, Component, DespawnRecursiveExt, Entity,
        IntoSystemConfigs, Query, States, With,
    },
    DefaultPlugins,
};

use crate::plugins::{
    BallPlugin, BrickPlugin, CameraPlugin, ColliderPlugin, PaddlePlugin, SplashScreenPlugin,
    WallsPlugin,
};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
enum GameState {
    InGame,

    #[default]
    SplashScreen,
}

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

fn despawn_by<T>(mut commands: Commands, entities_query: Query<Entity, With<T>>)
where
    T: Component,
{
    for entity in &entities_query {
        commands.entity(entity).despawn_recursive();
    }
}
