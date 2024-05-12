mod editor;

use bevy::app::{App};
use bevy::DefaultPlugins;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::editor::editor_gui::{EditorGuiPlugin, EditorSpace, SelectedSpriteSheet};
use crate::editor::editor_sprite_sheet::{EditorSpriteSheetPlugin, HitBox, HurtBox, SpriteSheets};

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
        .add_systems(Update, gizmos_selected_sprite)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let camera_transform = Transform::from_xyz(0.0, 0.0, 100.0);
    commands.insert_resource(OriginalCameraTransform(camera_transform.clone()));

    commands.spawn(Camera2dBundle::default());
}

fn display_selected_sprite_sheet(
    mut commands: Commands,
    mut sprite_sheets: ResMut<SpriteSheets>,
    selected_sprite_sheet: Res<SelectedSpriteSheet>,
    mut current_sprite_sheet_entity: ResMut<CurrentSpriteSheetEntity>,
) {
    if let Some(entity) = current_sprite_sheet_entity.entity {
        commands.entity(entity).despawn();
    }

    if let Some(id) = &selected_sprite_sheet.id {
        if let Some(frame_index) = &selected_sprite_sheet.frame_index {
            if let Some(sprite_sheet_atlas) = sprite_sheets.sheets.get_mut(id) {
                let texture_handle = sprite_sheet_atlas.texture_handle.clone();
                let mut entity = commands.spawn(SpriteSheetBundle {
                    texture: texture_handle,
                    atlas: TextureAtlas {
                        layout: sprite_sheet_atlas.handle.clone(),
                        index: *frame_index,
                    },
                    transform: Transform::from_scale(Vec3::splat(6.0)),
                    ..default()
                });

                if let Some(frame_index) = selected_sprite_sheet.frame_index {
                    if let Some(frame_data) = sprite_sheet_atlas.sprite_sheet_info.frames.get_mut(frame_index) {
                        for hit_box in &mut frame_data.hit_boxes {
                            entity.insert(HitBox {
                                size: hit_box.size,
                                offset: hit_box.offset,
                            });
                        }

                        for hurt_box in &mut frame_data.hurt_boxes {
                            entity.insert(HurtBox {
                                size: hurt_box.size,
                                offset: hurt_box.offset,
                            });
                        }
                    }
                }
                current_sprite_sheet_entity.entity = Some(entity.id());
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

fn gizmos_selected_sprite(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &HitBox, &HurtBox)>) {
    for (transform, hit_box, hurt_box) in query.iter() {
        let scale = transform.scale.truncate();

        let hit_box_size_scaled = hit_box.size * scale;
        let hit_box_offset_scaled = hit_box.offset * scale;

        let hurt_box_size_scaled = hurt_box.size * scale;
        let hurt_box_offset_scaled = hurt_box.offset * scale;

        gizmos.rect_2d(
            transform.translation.truncate() + hit_box_offset_scaled,
            0.0,
            hit_box_size_scaled,
            Color::RED,
        );

        gizmos.rect_2d(
            transform.translation.truncate() + hurt_box_offset_scaled,
            0.0,
            hurt_box_size_scaled,
            Color::GREEN,
        );
    }
}