use bevy::prelude::*;

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    InGame,
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PausedState {
    #[default]
    Paused,
    Resumed,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>();
        app.init_state::<PausedState>();
    }
}
