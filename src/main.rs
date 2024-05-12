mod editor;

use bevy::app::{App};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::editor::editor_gui::{EditorGuiPlugin, EditorSpace, SelectedSpriteSheet};
use crate::editor::editor_sprite_sheet::{EditorSpriteSheetPlugin, SpriteSheets};

#[derive(Default, Resource)]
struct CurrentSpriteSheetEntity {
    pub entity: Option<Entity>,
}

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EditorSpriteSheetPlugin)
        .add_plugins(EditorGuiPlugin)
        .insert_resource(CurrentSpriteSheetEntity::default())
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, display_selected_sprite_sheet)
        .add_systems(Update, update_camera_transform)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera_transform = Transform::from_xyz(0.0, 0.0, 100.0);
    commands.insert_resource(OriginalCameraTransform(camera_transform.clone()));

    commands.spawn(Camera2dBundle::default());
}

fn display_selected_sprite_sheet(
    mut commands: Commands,
    sprite_sheets: Res<SpriteSheets>,
    selected_sprite_sheet: Res<SelectedSpriteSheet>,
    mut current_sprite_sheet_entity: ResMut<CurrentSpriteSheetEntity>,
) {
    if let Some(entity) = current_sprite_sheet_entity.entity {
        commands.entity(entity).despawn();
    }

    if let Some(id) = &selected_sprite_sheet.id {
        if let Some(frame_index) = &selected_sprite_sheet.frame_index {
            if let Some(sprite_sheet_atlas) = sprite_sheets.sheets.get(id) {
                let texture_handle = sprite_sheet_atlas.texture_handle.clone();
                let entity = commands.spawn(SpriteSheetBundle {
                    texture: texture_handle,
                    atlas: TextureAtlas {
                        layout: sprite_sheet_atlas.handle.clone(),
                        index: *frame_index,
                    },
                    transform: Transform::from_scale(Vec3::splat(6.0)),
                    ..default()
                }).id();

                current_sprite_sheet_entity.entity = Some(entity);
            }
        }
    }
}

fn update_camera_transform(
    editor_space: Res<EditorSpace>,
    original_camera_transform: Res<OriginalCameraTransform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut transform = camera_query.single_mut();

    let window = windows.single();
    let right_taken = editor_space.right / window.width();

    transform.translation = original_camera_transform.translation
        + Vec3::new(
        (right_taken) * window.width() * 0.5,
        0.0,
        0.0,
    );
}