use bevy::{
    app::{App, Plugin},
    color::Color,
    input::ButtonInput,
    math::{Vec2, Vec3},
    prelude::{Commands, Component, KeyCode, OnEnter, Query, Res, Transform, With},
    sprite::Sprite,
    time::Time,
};

use crate::states::GameState;

use super::{
    collider_plugin::Collider,
    walls_plugin::{BOTTOM_WALL_Y_POS, LEFT_WALL_X_POS, RIGHT_WALL_X_POS, WALL_THICKNESS},
};

pub const PADDLE_Y_POS: f32 = BOTTOM_WALL_Y_POS + PADDLE_FLOOR_GAP_SIZE;

const PADDLE_FLOOR_GAP_SIZE: f32 = 40.0;
const PADDLE_PADDING: f32 = 10.0;
const PADDLE_SPEED: f32 = 500.0;
const PADDLE_SIZE: Vec2 = Vec2::new(225.0, 6.0);
const LEFT_BOUND: f32 =
    LEFT_WALL_X_POS + WALL_THICKNESS / 2.0 + PADDLE_SIZE.x / 2.0 + PADDLE_PADDING;
const RIGHT_BOUND: f32 =
    RIGHT_WALL_X_POS - WALL_THICKNESS / 2.0 - PADDLE_SIZE.x / 2.0 - PADDLE_PADDING;

pub struct PaddlePlugin;
impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), PaddlePlugin::setup);
    }
}

#[derive(Component)]
pub struct Paddle;

impl PaddlePlugin {
    pub fn move_paddle(
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
            paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();

        paddle_transform.translation.x = new_paddle_x_pos.clamp(LEFT_BOUND, RIGHT_BOUND);
    }

    fn setup(mut commands: Commands) {
        commands.spawn((
            Sprite::from_color(Color::srgb(0.04, 0.52, 0.76), PADDLE_SIZE),
            Transform::from_translation(Vec3::new(0.0, PADDLE_Y_POS, 0.0)),
            Paddle,
            Collider,
        ));
    }
}
