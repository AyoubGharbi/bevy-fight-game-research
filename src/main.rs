mod editor;

use bevy::app::{App};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::utils::petgraph::visit::Walker;
use crate::editor::editor_gui::{EditorGuiPlugin, SelectedSpriteSheet};
use crate::editor::editor_sprite_sheet::{EditorSpriteSheetPlugin, SpriteSheets};

#[derive(Default, Resource)]
struct CurrentSpriteSheetEntity {
    pub entity: Option<Entity>,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EditorSpriteSheetPlugin)
        .add_plugins(EditorGuiPlugin)
        .insert_resource(CurrentSpriteSheetEntity::default())
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, display_selected_sprite_sheet)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn display_selected_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_sheets: Res<SpriteSheets>,
    selected_sprite_sheet: Res<SelectedSpriteSheet>,
    texture_atlases: Res<Assets<TextureAtlasLayout>>,
    mut current_sprite_sheet_entity: ResMut<CurrentSpriteSheetEntity>,
) {

    // Clear the previous sprite sheet entity if it exists
    if let Some(entity) = current_sprite_sheet_entity.entity {
        commands.entity(entity).despawn();
    }

    if let Some(id) = &selected_sprite_sheet.id {
        if let Some(sprite_sheet_atlas) = sprite_sheets.sheets.get(id) {
            let texture_handle: Handle<Image> = asset_server.load(&sprite_sheet_atlas.image_path);
            // Assuming the layout is already loaded and ready to be used
            if let Some(layout) = texture_atlases.get(&sprite_sheet_atlas.handle) {
                let entity = commands.spawn(SpriteSheetBundle {
                    texture: texture_handle,
                    atlas: TextureAtlas {
                        layout: sprite_sheet_atlas.handle.clone(),
                        index: 0,
                    },
                    transform: Transform::from_scale(Vec3::splat(6.0)),
                    ..default()
                }).id();

                // Update the resource with the new entity
                current_sprite_sheet_entity.entity = Some(entity);
            }
        }
    }
}
