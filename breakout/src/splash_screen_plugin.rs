use bevy::{
    app::{App, Plugin, Update},
    asset::AssetServer,
    prelude::{
        in_state, BuildChildren, Commands, Component, Deref, DerefMut, ImageBundle,
        IntoSystemConfigs, NextState, OnEnter, OnExit, Res, ResMut, Resource,
    },
    time::{Time, Timer, TimerMode},
    ui::{Style, UiImage, Val},
    utils::default,
};

use crate::{states::GameState, systems::despawn_by, UserInterface};

#[derive(Component)]
pub struct SplashScreen;

#[derive(Deref, DerefMut, Resource)]
struct SplashScreenTimeout(Timer);

pub struct SplashScreenPlugin;
impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), SplashScreenPlugin::setup)
            .add_systems(
                Update,
                SplashScreenPlugin::advance_timeout.run_if(in_state(GameState::SplashScreen)),
            )
            .add_systems(OnExit(GameState::SplashScreen), despawn_by::<SplashScreen>);
    }
}

impl SplashScreenPlugin {
    fn advance_timeout(
        time: Res<Time>,
        mut game_state: ResMut<NextState<GameState>>,
        mut timeout: ResMut<SplashScreenTimeout>,
    ) {
        if timeout.tick(time.delta()).finished() {
            game_state.set(GameState::MainMenu);
        }
    }

    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((UserInterface::centered_container(), SplashScreen))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    image: UiImage::new(asset_server.load("textures/bevy_icon.png")),
                    style: Style {
                        width: Val::Px(200.0),
                        ..default()
                    },
                    ..default()
                });
            });

        commands.insert_resource(SplashScreenTimeout(Timer::from_seconds(
            1.0,
            TimerMode::Once,
        )));
    }
}
