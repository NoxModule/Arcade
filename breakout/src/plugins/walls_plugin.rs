mod wall_location;

use bevy::{
    app::{App, Plugin, Startup},
    color::Color,
    prelude::{Commands, Transform},
    sprite::{Sprite, SpriteBundle},
    utils::default,
};
use wall_location::WallLocation;

pub struct WallsPlugin;

const WALL_COLOR: Color = Color::srgb(0.306, 0.455, 0.729);

impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    spawn_wall(&mut commands, WallLocation::Bottom);
    spawn_wall(&mut commands, WallLocation::Left);
    spawn_wall(&mut commands, WallLocation::Right);
    spawn_wall(&mut commands, WallLocation::Top);
}

fn spawn_wall(commands: &mut Commands, location: WallLocation) {
    commands.spawn(SpriteBundle {
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
    });
}
