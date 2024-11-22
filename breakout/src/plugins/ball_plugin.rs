use bevy::{
    app::{App, Plugin, Startup},
    asset::Assets,
    color::Color,
    math::{Vec2, Vec3},
    prelude::{Circle, Commands, Component, Mesh, ResMut, Transform},
    sprite::{ColorMaterial, MaterialMesh2dBundle},
    utils::default,
};

use crate::components::Velocity;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

const BALL_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const BALL_DIAMETER: f32 = 30.;
const BALL_SPEED: f32 = 400.0;
const BALL_STARTING_POS: Vec3 = Vec3::new(0.0, -50.0, 1.0);
const BALL_INITIAL_DIRECTION: Vec2 = Vec2::new(0.5, -0.5);

#[derive(Component)]
struct Ball;

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
