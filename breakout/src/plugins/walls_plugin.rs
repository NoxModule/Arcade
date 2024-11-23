mod wall_location;

pub use wall_location::{
    BOTTOM_WALL_Y_POS, LEFT_WALL_X_POS, RIGHT_WALL_X_POS, TOP_WALL_Y_POS, WALL_THICKNESS,
};

use bevy::{
    app::{App, Plugin, Startup},
    color::Color,
    prelude::{Commands, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};
use wall_location::WallLocation;

use super::collider_plugin::Collider;

pub struct WallsPlugin;

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

const WALL_COLOR: Color = Color::srgb(0.306, 0.455, 0.729);

fn setup(mut commands: Commands) {
    spawn_wall(&mut commands, WallLocation::Bottom);
    spawn_wall(&mut commands, WallLocation::Left);
    spawn_wall(&mut commands, WallLocation::Right);
    spawn_wall(&mut commands, WallLocation::Top);
}

fn spawn_wall(commands: &mut Commands, location: WallLocation) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: WALL_COLOR,
                ..default()
            },
            transform: Transform {
                scale: location.size().extend(1.0),
                translation: location.position().extend(0.0),
                ..default()
            },
            ..default()
        },
        Collider,
    ));
}
