use bevy::{
    app::{App, AppExit, Plugin, Update},
    color::Color,
    prelude::{
        in_state, AppExtStates, BuildChildren, Button, ButtonBundle, Changed, Commands, Component,
        EventWriter, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, ResMut, TextBundle,
        With,
    },
    text::TextStyle,
    ui::{AlignItems, Interaction, JustifyContent, Style, UiRect, Val},
    utils::default,
};

use crate::{
    states::{GameState, MainMenuState},
    systems::despawn_by,
    UserInterface,
};

const BUTTON_DEFAULT_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
enum MainMenuAction {
    Play,
    Quit,
}

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), MainMenuPlugin::setup)
            .add_systems(
                OnEnter(MainMenuState::MainMenu),
                MainMenuPlugin::main_menu_setup,
            )
            .add_systems(
                Update,
                MainMenuPlugin::main_menu_action.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(OnExit(MainMenuState::MainMenu), despawn_by::<MainMenu>)
            .init_state::<MainMenuState>();
    }
}

impl MainMenuPlugin {
    fn main_menu_action(
        mut menu_state: ResMut<NextState<MainMenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
        mut app_exit_events: EventWriter<AppExit>,
        query: Query<(&Interaction, &MainMenuAction), (Changed<Interaction>, With<Button>)>,
    ) {
        for (interaction, main_menu_action) in &query {
            if *interaction == Interaction::Pressed {
                match main_menu_action {
                    MainMenuAction::Play => {
                        game_state.set(GameState::Game);
                        menu_state.set(MainMenuState::Disabled);
                    }
                    MainMenuAction::Quit => {
                        app_exit_events.send(AppExit::Success);
                    }
                }
            }
        }
    }

    fn main_menu_setup(mut commands: Commands) {
        let button_style = Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        };

        let button_text_style = TextStyle {
            font_size: 40.0,
            ..default()
        };

        commands
            .spawn((UserInterface::centered_container(), MainMenu))
            .with_children(|parent| {
                parent.spawn(
                    TextBundle::from_section(
                        "Breakout",
                        TextStyle {
                            font_size: 80.0,
                            ..default()
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    }),
                );

                parent
                    .spawn((
                        ButtonBundle {
                            background_color: BUTTON_DEFAULT_COLOR.into(),
                            style: button_style.clone(),
                            ..default()
                        },
                        MainMenuAction::Play,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section("Play", button_text_style.clone()));
                    });

                parent
                    .spawn((
                        ButtonBundle {
                            background_color: BUTTON_DEFAULT_COLOR.into(),
                            style: button_style.clone(),
                            ..default()
                        },
                        MainMenuAction::Quit,
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section("Quit", button_text_style.clone()));
                    });
            });
    }

    fn setup(mut main_menu_state: ResMut<NextState<MainMenuState>>) {
        main_menu_state.set(MainMenuState::MainMenu);
    }
}
