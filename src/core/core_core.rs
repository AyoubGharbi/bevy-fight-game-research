use bevy::app::App;
use std::fmt;

use crate::core::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CoreGuiPlugin)
            .insert_resource(GameState::default())
            .add_systems(Update, mode_switching_system);
    }
}

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


fn mode_switching_system(
    mut game_state: ResMut<GameState>,
    keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyE) {
        game_state.mode = GameMode::Editor;
    } else if keyboard.just_pressed(KeyCode::KeyG) {
        game_state.mode = GameMode::Game;
    }
}