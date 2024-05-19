use bevy::app::{App, Update};
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::{Vec2, Vec3};
use bevy::sprite::TextureAtlasLayout;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;

use std::fs;
use serde::{Deserialize, Serialize};

use crate::core::core_core::*;
use crate::editor::*;
use crate::editor::editor_gui::*;
use crate::editor::inspector::inspector_core::SelectedFrame;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EditorFrameData {
    pub hit_boxes: Vec<EditorHitBox>,
    pub hurt_boxes: Vec<EditorHurtBox>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EditorSpriteSheetInfo {
    pub id: String,
    pub image_path: String,
    pub sprite_sheet_width: usize,
    pub sprite_sheet_height: usize,
    pub tile_width: usize,
    pub tile_height: usize,
    pub columns: usize,
    pub rows: usize,
    pub frames: Vec<EditorFrameData>,
}


#[derive(Default, Resource)]
struct EditorSpriteSheet {
    pub entity: Option<Entity>,
}

#[derive(Default, Resource)]
pub(crate) struct EditorCamera {
    pub entity: Option<Entity>,
}

#[derive(Resource, Deref, DerefMut)]
struct EditorCameraTransform(Transform);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EditorSpriteSheetsData {
    pub sheets: Vec<EditorSpriteSheetInfo>,
}

#[derive(Resource)]
pub struct EditorSpriteSheetAtlas {
    pub handle: Handle<TextureAtlasLayout>,
    pub sprite_sheet_path: String,
    pub texture_handle: Handle<Image>,
    pub sprite_sheet_info: EditorSpriteSheetInfo,
}

#[derive(Resource)]
pub(crate) struct EditorSpriteSheets {
    pub(crate) sheets: HashMap<String, EditorSpriteSheetAtlas>,
}

#[derive(Default, Component, Serialize, Deserialize, Clone, Debug)]
pub struct EditorHitBox {
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Default, Component, Serialize, Deserialize, Clone, Debug)]
pub struct EditorHurtBox {
    pub size: Vec2,
    pub offset: Vec2,
}

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EditorGuiPlugin)
            .insert_resource(EditorSpriteSheets { sheets: HashMap::new() })
            .insert_resource(EditorSpriteSheet::default())
            .insert_resource(EditorCamera::default())
            .add_systems(Startup, load_sprite_sheets)
            .add_systems(Startup, spawn_editor_camera)
            .add_systems(Update, game_state_adapter_system)
            .add_systems(Update, display_selected_sprite_sheet)
            .add_systems(Update, update_camera_transform)
            .add_systems(Update, gizmos_hit_boxes_sprite)
            .add_systems(Update, gizmos_hurt_boxes_sprite);
    }
}

fn load_sprite_sheets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut sprite_sheets: ResMut<EditorSpriteSheets>,
) {
    let json_path = "assets/sprite_sheets.json";
    let sprite_sheet_data: EditorSpriteSheetsData = load_settings_from_file(json_path);

    for info in sprite_sheet_data.sheets {
        let tex_handle = asset_server.load(&info.image_path);
        let texture_atlas_layout = TextureAtlasLayout::from_grid(
            Vec2::new(info.tile_width as f32, info.tile_height as f32),
            info.columns,
            info.rows,
            None,
            None,
        );
        let texture_atlas_layout_handle = texture_atlases.add(texture_atlas_layout);
        let atlas_data = EditorSpriteSheetAtlas {
            sprite_sheet_info: info.clone(),
            handle: texture_atlas_layout_handle,
            sprite_sheet_path: info.image_path.clone(),
            texture_handle: tex_handle,
        };
        sprite_sheets.sheets.insert(info.id.clone(), atlas_data);
    }
}

fn spawn_editor_camera(
    mut commands: Commands,
    mut editor_camera: ResMut<EditorCamera>,
) {
    let camera_transform = Transform::from_xyz(0.0, 0.0, 100.0);
    commands.insert_resource(EditorCameraTransform(camera_transform));

    let entity = commands.spawn(Camera2dBundle::default());
    editor_camera.entity = Some(entity.id());
}

fn game_state_adapter_system(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut current_sprite_sheet_entity: ResMut<EditorSpriteSheet>,
    mut editor_camera_entity: ResMut<EditorCamera>) {
    match &game_state.mode {
        GameMode::Editor => {
            if editor_camera_entity.entity.is_none() {
                spawn_editor_camera(commands, editor_camera_entity);
            }
        }

        GameMode::Game => {
            if let Some(entity) = current_sprite_sheet_entity.entity.take() {
                commands.entity(entity).despawn();
            }
            if let Some(entity) = editor_camera_entity.entity.take() {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn display_selected_sprite_sheet(
    mut commands: Commands,
    mut sprite_sheets: ResMut<EditorSpriteSheets>,
    mut selected_frame: ResMut<SelectedFrame>,
    mut current_sprite_sheet: ResMut<EditorSpriteSheet>,
    game_state: Res<GameState>,
) {
    if game_state.mode != GameMode::Editor {
        return;
    }

    if let Some(entity) = current_sprite_sheet.entity {
        commands.entity(entity).despawn();
    }


    if let Some(id) = &selected_frame.sprite_sheet_id {
        if let Some(frame_index) = &selected_frame.frame_index {
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

                if let Some(frame_index) = selected_frame.frame_index {
                    if let Some(sheet_info) = &mut selected_frame.sheet_info {
                        if let Some(frame_data) = sheet_info.frames.get_mut(frame_index) {
                            for hit_box in frame_data.hit_boxes.iter_mut() {
                                println!("Hit box : {}", hit_box.size);
                                entity.insert(hit_box.clone());
                            }

                            for hurt_box in frame_data.hurt_boxes.iter_mut() {
                                println!("Hurt box : {}", hurt_box.size);
                                entity.insert(hurt_box.clone());
                            }
                        }
                    }
                }
                current_sprite_sheet.entity = Some(entity.id());
            }
        }
    }
}

fn update_camera_transform(
    editor_space: Res<EditorGuiSpace>,
    original_camera_transform: Res<EditorCameraTransform>,
    windows: Query<&Window, With<PrimaryWindow>>,
    editor_camera_entity: Res<EditorCamera>,
    mut transforms: Query<&mut Transform>,
    game_state: Res<GameState>,
) {
    if game_state.mode != GameMode::Editor {
        return;
    }

    if let Some(camera_entity) = editor_camera_entity.entity {
        if let Ok(mut transform) = transforms.get_mut(camera_entity) {
            let window = windows.single();
            let right_taken = editor_space.right / window.width();
            let left_taken = editor_space.left / window.width();

            let translation_x = if game_state.mode == GameMode::Editor {
                (right_taken - left_taken) * window.width() * 0.5
            } else {
                0.0
            };

            transform.translation = original_camera_transform.translation
                + Vec3::new(
                translation_x,
                0.0,
                0.0,
            );
        }
    }
}


fn gizmos_hurt_boxes_sprite(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &EditorHurtBox)>,
    game_state: Res<GameState>,
) {
    if game_state.mode != GameMode::Editor {
        return;
    }
    for (transform, hurt_box) in query.iter() {
        let scale = transform.scale.truncate();

        let hurt_box_size_scaled = hurt_box.size * scale;
        let hurt_box_offset_scaled = hurt_box.offset * scale;

        gizmos.rect_2d(
            transform.translation.truncate() + hurt_box_offset_scaled,
            0.0,
            hurt_box_size_scaled,
            Color::GREEN,
        );
    }
}

fn gizmos_hit_boxes_sprite(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &EditorHitBox)>,
    game_state: Res<GameState>,
) {
    if game_state.mode != GameMode::Editor {
        return;
    }
    for (transform, hit_box) in query.iter() {
        let scale = transform.scale.truncate();

        let hit_box_size_scaled = hit_box.size * scale;
        let hit_box_offset_scaled = hit_box.offset * scale;

        gizmos.rect_2d(
            transform.translation.truncate() + hit_box_offset_scaled,
            0.0,
            hit_box_size_scaled,
            Color::RED,
        );
    }
}

fn load_settings_from_file<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let data = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse JSON")
}

pub fn save_settings_to_file<T: Serialize>(path: &str, data: &T) {
    let serialized_data = serde_json::to_string_pretty(data)
        .expect("Unable to serialize data");
    fs::write(path, serialized_data)
        .expect("Unable to write to file");
}