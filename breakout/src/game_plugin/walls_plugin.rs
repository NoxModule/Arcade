mod wall_location;

use bevy::{
    app::{App, Plugin},
    color::Color,
    prelude::{Commands, OnEnter, Transform},
    sprite::Sprite,
    utils::default,
};

use crate::states::GameState;

use super::collider_plugin::Collider;

pub use self::wall_location::{
    BOTTOM_WALL_Y_POS, LEFT_WALL_X_POS, RIGHT_WALL_X_POS, TOP_WALL_Y_POS, WALL_THICKNESS,
};

use self::wall_location::WallLocation;

pub struct WallsPlugin;
impl Plugin for WallsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), WallsPlugin::setup);
    }
}

impl WallsPlugin {
    fn setup(mut commands: Commands) {
        WallsPlugin::spawn_wall(&mut commands, WallLocation::Bottom);
        WallsPlugin::spawn_wall(&mut commands, WallLocation::Left);
        WallsPlugin::spawn_wall(&mut commands, WallLocation::Right);
        WallsPlugin::spawn_wall(&mut commands, WallLocation::Top);
    }

    fn spawn_wall(commands: &mut Commands, location: WallLocation) {
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                ..default()
            },
            Transform {
                scale: location.size().extend(1.0),
                translation: location.position().extend(0.0),
                ..default()
            },
            Collider,
        ));
    }
}
