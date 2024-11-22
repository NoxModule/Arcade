use bevy::{
    app::{App, FixedUpdate, Plugin, Startup},
    color::Color,
    input::ButtonInput,
    math::{Vec2, Vec3},
    prelude::{Commands, Component, IntoSystemConfigs, KeyCode, Query, Res, Transform, With},
    sprite::{Sprite, SpriteBundle},
    time::Time,
    utils::default,
};

use crate::components::Velocity;

use super::walls_plugin::{BOTTOM_WALL_Y_POS, LEFT_WALL_X_POS, RIGHT_WALL_X_POS, WALL_THICKNESS};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, (apply_velocity, move_paddle).chain());
    }
}

const PADDLE_COLOR: Color = Color::srgb(0.13, 0.13, 0.13);
const PADDLE_FLOOR_GAP_SIZE: f32 = 60.0;
const PADDLE_PADDING: f32 = 10.0;
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const LEFT_BOUND: f32 =
    LEFT_WALL_X_POS + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
const RIGHT_BOUND: f32 =
    RIGHT_WALL_X_POS - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

#[derive(Component)]
struct Paddle;

fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}

fn move_paddle(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Paddle>>,
) {
    let mut paddle_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard.pressed(KeyCode::KeyA) {
        direction -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction += 1.0;
    }

    let new_paddle_x_pos =
        paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_seconds();

    paddle_transform.translation.x = new_paddle_x_pos.clamp(LEFT_BOUND, RIGHT_BOUND);
}

fn setup(mut commands: Commands) {
    let paddle_y_pos = BOTTOM_WALL_Y_POS + PADDLE_FLOOR_GAP_SIZE;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            transform: Transform {
                scale: PADDLE_SIZE.extend(1.0),
                translation: Vec3::new(0.0, paddle_y_pos, 0.0),
                ..default()
            },
            ..default()
        },
        Paddle,
    ));
}
