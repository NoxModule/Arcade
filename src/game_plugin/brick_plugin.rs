use bevy::{
    app::{App, Plugin},
    color::Color,
    math::Vec2,
    prelude::{Commands, Component, OnEnter, Transform},
    sprite::Sprite,
};

use crate::states::GameState;

use super::{
    collider_plugin::Collider,
    walls_plugin::{LEFT_WALL_X_POS, TOP_WALL_Y_POS},
};

const BRICK_COLUMN_COUNT: usize = 14;
const BRICK_ROW_COUNT: usize = 8;
const BRICK_PADDING: f32 = 2.0;
const BRICK_SIZE: Vec2 = Vec2::new(17.0, 5.0);

const BRICK_ROW_COLORS: [Color; 4] = [
    Color::srgb(0.88, 0.15, 0.0),
    Color::srgb(1.0, 0.66, 0.0),
    Color::srgb(0.0, 0.75, 0.25),
    Color::srgb(0.99, 1.0, 0.15),
];

pub struct BrickPlugin;
impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), BrickPlugin::setup);
    }
}

#[derive(Component)]
pub struct Brick;

impl BrickPlugin {
    fn setup(mut commands: Commands) {
        let offset_x = LEFT_WALL_X_POS + BRICK_SIZE.x / 2.0 - BRICK_PADDING;
        let offset_y = TOP_WALL_Y_POS - 50.0;

        for row in 0..BRICK_ROW_COUNT {
            for column in 0..BRICK_COLUMN_COUNT {
                let brick_position = Vec2::new(
                    offset_x + column as f32 * (BRICK_SIZE.x + BRICK_PADDING),
                    offset_y - row as f32 * (BRICK_SIZE.y + BRICK_PADDING),
                );

                commands.spawn((
                    Sprite::from_color(
                        BRICK_ROW_COLORS[(row / 2) % BRICK_ROW_COLORS.len()].clone(),
                        BRICK_SIZE,
                    ),
                    Transform::from_translation(brick_position.extend(-1.0)),
                    Brick,
                    Collider,
                ));
            }
        }
    }
}
