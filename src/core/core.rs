use std::fmt;

use bevy::app::App;
use bevy::prelude::{ButtonInput, Commands, KeyCode, Plugin, Res, ResMut, Resource, Startup, Update};

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
            .add_systems(Startup, initialization_system)
            .add_systems(Update, mode_switching_system)
            .add_systems(Update, mode_switching_lifecycle_system);
    }
}

fn initialization_system(mut commands: Commands) {
    commands.insert_resource(GameState::default());
}

fn mode_switching_system(mut game_state: ResMut<GameState>, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        game_state.mode = GameMode::Editor;
    } else if keyboard.just_pressed(KeyCode::KeyG) {
        game_state.mode = GameMode::Game;
    }
}

fn mode_switching_lifecycle_system(mut commands: Commands, game_state: Res<GameState>) {
    match &game_state.mode {
        GameMode::Editor => {
        }
        GameMode::Game => {}
    }
}

fn save_state_system() {}

fn load_state_system() {}