use bevy::{
    app::{App, Plugin, Startup},
    color::Color,
    math::{Vec2, Vec3},
    prelude::{Commands, Component, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use super::{
    collider_plugin::Collider,
    paddle_plugin::PADDLE_Y_POS,
    walls_plugin::{LEFT_WALL_X_POS, RIGHT_WALL_X_POS, TOP_WALL_Y_POS},
};

const BRICK_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const BRICK_PADDING: f32 = 5.0;
const BRICK_SIZE: Vec2 = Vec2::new(100.0, 30.0);
const BRICKS_FIELD_BOTTOM: f32 = PADDLE_Y_POS + BRICKS_PADDLE_PADDING;
const BRICKS_FIELD_HEIGHT: f32 = TOP_WALL_Y_POS - BRICKS_FIELD_BOTTOM - BRICKS_CEILING_PADDING;
const BRICKS_FIELD_WIDTH: f32 = (RIGHT_WALL_X_POS - LEFT_WALL_X_POS) - 2.0 * BRICKS_WALLS_PADDING;
const BRICKS_CEILING_PADDING: f32 = 20.0;
const BRICKS_PADDLE_PADDING: f32 = 270.0;
const BRICKS_WALLS_PADDING: f32 = 20.0;

#[derive(Component)]
pub struct Brick;

pub struct BrickPlugin;

impl BrickPlugin {
    fn setup(mut commands: Commands) {
        let n_columns = (BRICKS_FIELD_WIDTH / (BRICK_SIZE.x + BRICK_PADDING)).floor() as usize;
        let n_rows = (BRICKS_FIELD_HEIGHT / (BRICK_SIZE.y + BRICK_PADDING)).floor() as usize;
        let n_vertical_gaps = n_columns - 1;

        let center_of_bricks = (LEFT_WALL_X_POS + RIGHT_WALL_X_POS) / 2.0;
        let left_edge_of_bricks = center_of_bricks
            - (n_columns as f32 / 2.0 * BRICK_SIZE.x)
            - n_vertical_gaps as f32 / 2.0 * BRICK_PADDING;

        let offset_x = left_edge_of_bricks + BRICK_SIZE.x / 2.0;
        let offset_y = BRICKS_FIELD_BOTTOM + BRICK_SIZE.y / 2.0;

        for row in 0..n_rows {
            for column in 0..n_columns {
                let brick_position = Vec2::new(
                    offset_x + column as f32 * (BRICK_SIZE.x + BRICK_PADDING),
                    offset_y + row as f32 * (BRICK_SIZE.y + BRICK_PADDING),
                );

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: BRICK_COLOR,
                            ..default()
                        },
                        transform: Transform {
                            translation: brick_position.extend(0.0),
                            scale: Vec3::new(BRICK_SIZE.x, BRICK_SIZE.y, 1.0),
                            ..default()
                        },
                        ..default()
                    },
                    Brick,
                    Collider,
                ));
            }
        }
    }
}

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, BrickPlugin::setup);
    }
}
