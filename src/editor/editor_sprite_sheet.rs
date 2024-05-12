use std::fs;

use bevy::app::App;
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Component, Image, Plugin, Res, ResMut, Resource, Startup};
use bevy::sprite::TextureAtlasLayout;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

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
            .add_systems(Startup, load_sprite_sheets);
        // .add_systems(Update, debug_sprite_sheets_loaded);
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


fn debug_sprite_sheets_loaded(
    sprite_sheets: Res<SpriteSheets>
) {
    for (id, handle) in sprite_sheets.sheets.iter() {
        println!("Sprite Sheet ID: {}, Handle: {:?}", id, handle.sprite_sheet_path);
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