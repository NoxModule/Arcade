use bevy::prelude::States;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    Game,

    MainMenu,

    #[default]
    SplashScreen,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum MainMenuState {
    #[default]
    Disabled,

    MainMenu,
}
