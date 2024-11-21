use bevy::{
    app::{App, Plugin, Startup},
    math::Vec3,
    prelude::{Commands, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};

use crate::constants::{BOTTOM_WALL_Y_POS, PADDLE_COLOR, PADDLE_FLOOR_GAP_SIZE, PADDLE_SIZE};

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    let paddle_y_pos = BOTTOM_WALL_Y_POS + PADDLE_FLOOR_GAP_SIZE;

    commands.spawn(SpriteBundle {
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
    });
}
