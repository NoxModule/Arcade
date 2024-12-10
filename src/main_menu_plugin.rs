use bevy::{
    app::{App, AppExit, Plugin, Update},
    color::Color,
    prelude::{
        in_state, AppExtStates, BuildChildren, Button, Changed, ChildBuild, ClearColor, Commands,
        Component, EventWriter, IntoSystemConfigs, NextState, OnEnter, OnExit, Query, Res, ResMut,
        Text, With,
    },
    text::TextFont,
    ui::{AlignItems, BackgroundColor, Interaction, JustifyContent, Node, UiRect, Val},
    utils::default,
};

use crate::{
    cli_arguments::CliArguments,
    states::{GameState, MainMenuState},
    systems::despawn_by,
    UserInterface,
};

const BUTTON_DEFAULT_COLOR: Color = Color::srgb(0.17, 0.24, 0.24);

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
        let button_node = Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Px(20.0)),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        };

        let button_text_font = TextFont {
            font_size: 40.0,
            ..default()
        };

        commands.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));

        commands
            .spawn((UserInterface::centered_container(), MainMenu))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("Arcade"),
                    TextFont {
                        font_size: 80.0,
                        ..default()
                    },
                ));

                parent
                    .spawn((
                        Button::default(),
                        button_node.clone(),
                        BackgroundColor::from(BUTTON_DEFAULT_COLOR),
                        MainMenuAction::Play,
                    ))
                    .with_children(|parent| {
                        parent.spawn((Text::new("Play"), button_text_font.clone()));
                    });

                parent
                    .spawn((
                        Button::default(),
                        button_node.clone(),
                        BackgroundColor::from(BUTTON_DEFAULT_COLOR),
                        MainMenuAction::Quit,
                    ))
                    .with_children(|parent| {
                        parent.spawn((Text::new("Quit"), button_text_font.clone()));
                    });
            });
    }

    fn setup(
        mut menu_state: ResMut<NextState<MainMenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
        cli_arguments: Res<CliArguments>,
    ) {
        if cli_arguments.skip_menu {
            game_state.set(GameState::Game);
        } else {
            menu_state.set(MainMenuState::MainMenu);
        }
    }
}
