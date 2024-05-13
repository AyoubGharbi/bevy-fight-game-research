use std::fs;

use bevy::app::{App, Update};
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Camera, Camera2dBundle, Color, Commands, Component, default, Deref, DerefMut, Entity, Gizmos, Image, Plugin, Query, Res, ResMut, Resource, SpriteSheetBundle, Startup, TextureAtlas, Transform, Window, With};
use bevy::sprite::TextureAtlasLayout;
use bevy::utils::HashMap;
use bevy::window::PrimaryWindow;
use serde::{Deserialize, Serialize};

use crate::core::core::{GameMode, GameState};
use crate::editor::editor_gui::{EditorSpace, SelectedSpriteSheet};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FrameData {
    pub hit_boxes: Vec<HitBox>,
    pub hurt_boxes: Vec<HurtBox>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpriteSheetInfo {
    pub id: String,
    pub image_path: String,
    pub sprite_sheet_width: usize,
    pub sprite_sheet_height: usize,
    pub tile_width: usize,
    pub tile_height: usize,
    pub columns: usize,
    pub rows: usize,
    pub frames: Vec<FrameData>,
}


#[derive(Default, Resource)]
struct CurrentSpriteSheetEntity {
    pub entity: Option<Entity>,
}

#[derive(Resource, Deref, DerefMut)]
struct OriginalCameraTransform(Transform);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SpriteSheetsData {
    pub sheets: Vec<SpriteSheetInfo>,
}

#[derive(Resource)]
pub struct SpriteSheetAtlas {
    pub handle: Handle<TextureAtlasLayout>,
    pub sprite_sheet_path: String,
    pub texture_handle: Handle<Image>,
    pub sprite_sheet_info: SpriteSheetInfo,
}

#[derive(Resource)]
pub(crate) struct SpriteSheets {
    pub(crate) sheets: HashMap<String, SpriteSheetAtlas>,
}

#[derive(Default, Component, Serialize, Deserialize, Clone, Debug)]
pub struct HitBox {
    pub size: Vec2,
    pub offset: Vec2,
}

#[derive(Default, Component, Serialize, Deserialize, Clone, Debug)]
pub struct HurtBox {
    pub size: Vec2,
    pub offset: Vec2,
}

pub struct EditorSpriteSheetPlugin;

impl Plugin for EditorSpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpriteSheets { sheets: HashMap::new() })
            .insert_resource(CurrentSpriteSheetEntity::default())
            .add_systems(Startup, load_sprite_sheets)
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, display_selected_sprite_sheet)
            .add_systems(Update, update_camera_transform)
            .add_systems(Update, gizmos_selected_sprite);
    }
}

fn load_sprite_sheets(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut sprite_sheets: ResMut<SpriteSheets>,
) {
    let json_path = "assets/sprite_sheets.json";
    let sprite_sheet_data: SpriteSheetsData = load_settings_from_file(json_path);

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
        let atlas_data = SpriteSheetAtlas {
            sprite_sheet_info: info.clone(),
            handle: texture_atlas_layout_handle,
            sprite_sheet_path: info.image_path.clone(),
            texture_handle: tex_handle,
        };
        sprite_sheets.sheets.insert(info.id.clone(), atlas_data);
    }
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
    game_state: Res<GameState>,
) {
    if game_state.mode != GameMode::Editor {
        return;
    }

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
    game_state: Res<GameState>,
) {
    let is_editor = game_state.mode == GameMode::Editor;

    let mut transform = camera_query.single_mut();

    let window = windows.single();
    let right_taken = editor_space.right / window.width();

    let translation_x = if is_editor {
        (right_taken) * window.width() * 0.5
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

fn gizmos_selected_sprite(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &HitBox, &HurtBox)>,
    game_state: Res<GameState>,
) {
    if game_state.mode != GameMode::Editor {
        return;
    }
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