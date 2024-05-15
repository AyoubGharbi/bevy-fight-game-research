use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::*;

mod core;

use crate::core::core_core::*;

mod editor;

use crate::editor::editor_core::EditorPlugin;

mod game;

use crate::game::game_core::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CorePlugin)
        .add_plugins(EditorPlugin)
        .add_plugins(GamePlugin)
        .run();
}