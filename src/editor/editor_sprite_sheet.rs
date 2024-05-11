use std::fs;

use bevy::app::App;
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::math::Vec2;
use bevy::prelude::{Commands, Component, Image, Plugin, Res, ResMut, Resource, Startup, Update};
use bevy::sprite::TextureAtlasLayout;
use bevy::utils::HashMap;
use bevy::utils::petgraph::visit::Walker;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SpriteSheetInfo {
    id: String,
    image_path: String,
    tile_width: usize,
    tile_height: usize,
    columns: usize,
    rows: usize,
}

#[derive(Resource)]
pub struct SpriteSheetAtlas {
    pub(crate) handle: Handle<TextureAtlasLayout>,
    pub(crate) image_path: String,
    pub(crate) texture_handle: Handle<Image>,
}

#[derive(Resource)]
pub(crate) struct SpriteSheets {
    pub(crate) sheets: HashMap<String, SpriteSheetAtlas>,
}

#[derive(Component, Serialize, Deserialize, Clone)]
struct HitBox {
    size: Vec2,
    offset: Vec2,
}

#[derive(Component, Serialize, Deserialize, Clone)]
struct HurtBox {
    size: Vec2,
    offset: Vec2,
}

pub struct EditorSpriteSheetPlugin;

impl Plugin for EditorSpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpriteSheets { sheets: HashMap::new() })
            .add_systems(Startup, load_sprite_sheets)
            .add_systems(Update, debug_sprite_sheets_loaded);
    }
}

fn load_sprite_sheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut sprite_sheets: ResMut<SpriteSheets>,
) {
    let json_path = "assets/sprite_sheets.json";
    let sprite_sheet_infos: Vec<SpriteSheetInfo> = load_settings_from_file(json_path);

    for info in sprite_sheet_infos {
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
            handle: texture_atlas_layout_handle,
            image_path: info.image_path.clone(),
            texture_handle: tex_handle,
        };
        sprite_sheets.sheets.insert(info.id.clone(), atlas_data);
    }
}


fn debug_sprite_sheets_loaded(
    sprite_sheets: Res<SpriteSheets>
) {
    for (id, handle) in sprite_sheets.sheets.iter() {
        println!("Sprite Sheet ID: {}, Handle: {:?}", id, handle.image_path);
    }
}

fn load_settings_from_file<T: for<'de> Deserialize<'de>>(path: &str) -> T {
    let data = fs::read_to_string(path).expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse JSON")
}