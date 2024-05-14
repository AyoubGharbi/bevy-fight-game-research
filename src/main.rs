use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::*;

use crate::core::core::CorePlugin;
use crate::editor::editor_core::EditorPlugin;
use crate::game::game_core::GamePlugin;

mod editor;
mod core;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CorePlugin)
        .add_plugins(EditorPlugin)
        .add_plugins(GamePlugin)
        .run();
}