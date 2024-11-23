use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    color::Color,
    math::{Vec2, Vec3},
    prelude::{Circle, Commands, Component, Mesh, Query, Res, ResMut, Transform},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    time::Time,
    utils::default,
};

use crate::components::Velocity;

#[derive(Component)]
pub struct Ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, BallPlugin::setup);
    }
}

pub const BALL_DIAMETER: f32 = 30.0;

const BALL_COLOR: Color = Color::srgb(0.5, 0.13, 0.13);
const BALL_SPEED: f32 = 400.0;
const BALL_STARTING_POS: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

impl BallPlugin {
    pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
        for (mut transform, velocity) in &mut query {
            transform.translation.x += velocity.x * time.delta_seconds();
            transform.translation.y += velocity.y * time.delta_seconds();
        }
    }

    fn setup(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        commands.spawn((
            MaterialMesh2dBundle {
                material: materials.add(BALL_COLOR),
                mesh: meshes.add(Circle::default()).into(),
                transform: Transform::from_translation(BALL_STARTING_POS)
                    .with_scale(Vec2::splat(BALL_DIAMETER).extend(1.)),
                ..default()
            },
            Ball,
            Velocity(BALL_INITIAL_DIRECTION.normalize() * BALL_SPEED),
        ));
    }
}
