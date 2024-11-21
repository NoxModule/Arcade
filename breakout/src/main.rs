mod constants;

use bevy::{
    app::{App, Startup},
    math::Vec3,
    prelude::{Camera2dBundle, ClearColor, Commands, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::default,
    DefaultPlugins,
};

use crate::constants::{
    BACKGROUND_COLOR, BOTTOM_WALL_Y_POS, PADDLE_COLOR, PADDLE_FLOOR_GAP_SIZE, PADDLE_SIZE,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

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
