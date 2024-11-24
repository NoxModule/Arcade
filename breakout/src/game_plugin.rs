mod ball_plugin;
mod brick_plugin;
mod collider_plugin;
mod components;
mod paddle_plugin;
mod walls_plugin;

use bevy::{
    app::{App, FixedUpdate, Plugin},
    prelude::{in_state, IntoSystemConfigs},
};

use crate::states::GameState;

use self::{
    ball_plugin::BallPlugin, brick_plugin::BrickPlugin, collider_plugin::ColliderPlugin,
    paddle_plugin::PaddlePlugin, walls_plugin::WallsPlugin,
};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BallPlugin,
            BrickPlugin,
            ColliderPlugin,
            PaddlePlugin,
            WallsPlugin,
        ))
        .add_systems(
            FixedUpdate,
            (
                BallPlugin::apply_velocity.run_if(in_state(GameState::Game)),
                PaddlePlugin::move_paddle.run_if(in_state(GameState::Game)),
                ColliderPlugin::check_collisions.run_if(in_state(GameState::Game)),
            )
                .chain(),
        );
    }
}
