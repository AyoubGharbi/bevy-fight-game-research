mod editor;
use bevy::app::{App};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::utils::petgraph::visit::Walker;
use crate::editor::editor_gui::EditorGuiPlugin;
use crate::editor::editor_sprite_sheet::{EditorSpriteSheetPlugin, SpriteSheets};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EditorSpriteSheetPlugin)
        .add_plugins(EditorGuiPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, load_sprite_sheet)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn load_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_sheets: Res<SpriteSheets>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
) {
    if let Some((_, sprite_sheet_atlas)) = sprite_sheets.sheets.iter().next() {
        // Load the texture associated with the image path
        let texture_handle: Handle<Image> = asset_server.load(&sprite_sheet_atlas.image_path);

        // Here we assume the atlas layout is already loaded into texture_atlases,
        // and we simply reference it in the TextureAtlas component
        if let Some(layout) = texture_atlases.get(&sprite_sheet_atlas.handle) {
            // Now, we need to create a TextureAtlas from the layout to use in the bundle
            // Assuming TextureAtlas can directly use TextureAtlasLayout (if not, adjust accordingly)
            commands.spawn(SpriteSheetBundle {
                texture: texture_handle,
                atlas: TextureAtlas {
                    layout: sprite_sheet_atlas.handle.clone(),
                    index: 0,  // Starting index
                },
                transform: Transform::from_scale(Vec3::splat(6.0)),
                ..default()
            });
        }
    }
}