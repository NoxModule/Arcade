use bevy::{
    app::{App, Plugin},
    color::Color,
    math::{Vec2, Vec3},
    prelude::{Commands, Component, OnEnter, Query, Res, Transform},
    sprite::Sprite,
    time::Time,
};

use crate::states::GameState;

use super::components::Velocity;

pub const BALL_SIZE: Vec2 = Vec2::new(7.0, 5.0);

const BALL_SPEED: f32 = 200.0;
const BALL_STARTING_POS: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

#[derive(Component)]
pub struct Ball;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), BallPlugin::setup);
    }
}

impl BallPlugin {
    pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
        for (mut transform, velocity) in &mut query {
            transform.translation.x += velocity.x * time.delta_secs();
            transform.translation.y += velocity.y * time.delta_secs();
        }
    }

    fn setup(mut commands: Commands) {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.8, 0.8, 0.8), BALL_SIZE),
            Transform::from_translation(BALL_STARTING_POS),
            Ball,
            Velocity(BALL_INITIAL_DIRECTION.normalize() * BALL_SPEED),
        ));
    }
}
