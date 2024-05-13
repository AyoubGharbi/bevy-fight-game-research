use std::fmt;
use bevy::app::App;
use bevy::prelude::{Commands, Plugin, Res, Resource, Startup};
use crate::core::core_gui::CoreGuiPlugin;

#[derive(Resource)]
pub struct GameState {
    pub mode: GameMode,
}

impl fmt::Display for GameMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            GameMode::Editor => "Editor",
            GameMode::Game => "Game",
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum GameMode {
    Editor,
    Game,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            mode: GameMode::Editor,
        }
    }
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreGuiPlugin)
            .add_systems(Startup, initialization_system);
    }
}

fn initialization_system(mut commands: Commands) {
    commands.insert_resource(GameState::default());
}

fn mode_switching_system(mut commands: Commands, game_state: Res<GameState>) {
    match &game_state.mode {
        GameMode::Editor => {}
        GameMode::Game => {}
    }
}

fn save_state_system() {}

fn load_state_system() {}