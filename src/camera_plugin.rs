use bevy::{
    app::{App, Plugin, Startup},
    prelude::{Camera2d, Commands},
};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, CameraPlugin::setup);
    }
}

impl CameraPlugin {
    fn setup(mut commands: Commands) {
        commands.spawn(Camera2d::default());
    }
}
