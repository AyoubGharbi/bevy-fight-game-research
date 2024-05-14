use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::*;

use crate::core::core::CorePlugin;
use crate::editor::editor_core::EditorSpriteSheetPlugin;
use crate::editor::editor_gui::EditorGuiPlugin;
use crate::game::game_core::GamePlugin;

mod editor;
mod core;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CorePlugin)
        .add_plugins(EditorSpriteSheetPlugin)
        .add_plugins(EditorGuiPlugin)
        .add_plugins(GamePlugin)
        .run();
}