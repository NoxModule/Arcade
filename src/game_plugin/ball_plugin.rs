use bevy::{
    app::{App, Plugin},
    asset::Assets,
    color::Color,
    math::{Vec2, Vec3},
    prelude::{Circle, Commands, Component, Mesh, Mesh2d, OnEnter, Query, Res, ResMut, Transform},
    sprite::{ColorMaterial, MeshMaterial2d},
    time::Time,
};

use crate::states::GameState;

use super::components::Velocity;

pub const BALL_DIAMETER: f32 = 30.0;

const BALL_SPEED: f32 = 400.0;
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

    fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(Color::WHITE)),
            Transform::from_translation(BALL_STARTING_POS)
                .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
            Ball,
            Velocity(BALL_INITIAL_DIRECTION.normalize() * BALL_SPEED),
        ));
    }
}
