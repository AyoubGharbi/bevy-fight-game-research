use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::prelude::*;

use crate::core::core::CorePlugin;
use crate::editor::editor_gui::EditorGuiPlugin;
use crate::editor::editor_sprite_sheet::EditorSpriteSheetPlugin;

mod editor;
mod core;

#[derive(Default, Resource)]
struct CurrentSpriteSheetEntity {
    pub entity: Option<Entity>,
}

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CorePlugin)
        .add_plugins(EditorSpriteSheetPlugin)
        .add_plugins(EditorGuiPlugin)
        .run();
}

