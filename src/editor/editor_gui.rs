use bevy::app::{App, Plugin};
use bevy::prelude::{Res, ResMut, Resource, Update};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui::egui::{Image, TextureFilter, TextureOptions, TextureWrapMode};

use crate::editor::editor_sprite_sheet::SpriteSheets;

#[derive(Resource, Default)]
pub struct SelectedSpriteSheet {
    pub id: Option<String>,
    pub frame_index: Option<usize>,
}

pub struct EditorGuiPlugin;

impl Plugin for EditorGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .insert_resource(SelectedSpriteSheet::default())
            .add_systems(Update, load_sprite_sheets);
    }
}

fn load_sprite_sheets(
    mut egui_contexts: EguiContexts,
    sprite_sheets: Res<SpriteSheets>,
    mut selected_sprite_sheet: ResMut<SelectedSpriteSheet>, ) {
    let ctx = egui_contexts.ctx_mut();
    egui_extras::install_image_loaders(&ctx);
    egui::SidePanel::left("side_panel")
        .resizable(true).show(ctx, |ui| {
        ui.collapsing("Loaded Sprite Sheets", |ui| {
            for (id, _sprite_sheet_atlas) in sprite_sheets.sheets.iter() {
                if (ui.button(id)).clicked() {
                    selected_sprite_sheet.id = Some(id.clone());
                }
            }
        });

        ui.collapsing("Selected Sprite Sheet", |ui| {
            if let Some(id) = &selected_sprite_sheet.id {
                if let Some(sprite_sheet_atlas) = sprite_sheets.sheets.get(id) {
                    let ui_path = sprite_sheet_atlas.sprite_sheet_path.clone();
                    let scale = 64.0 / sprite_sheet_atlas.sprite_sheet_info.tile_width as f32;
                    let scaled_width = scale * sprite_sheet_atlas.sprite_sheet_info.tile_width as f32;
                    let scaled_height = scale * sprite_sheet_atlas.sprite_sheet_info.tile_height as f32;

                    let total_width = sprite_sheet_atlas.sprite_sheet_info.columns as f32 * scaled_width;
                    let total_height = sprite_sheet_atlas.sprite_sheet_info.rows as f32 * scaled_height;

                    let image = Image::new(format!("file://assets/{ui_path}"))
                        .texture_options(TextureOptions {
                            magnification: TextureFilter::Nearest,
                            minification: TextureFilter::Nearest,
                            wrap_mode: TextureWrapMode::ClampToEdge,
                        }).fit_to_exact_size(egui::vec2(total_width, total_height));

                    ui.add(image);

                    // Render each tile as a clickable image
                    for row in 0..sprite_sheet_atlas.sprite_sheet_info.rows {
                        ui.horizontal(|ui| {
                            for col in 0..sprite_sheet_atlas.sprite_sheet_info.columns {
                                let frame_index = row * sprite_sheet_atlas.sprite_sheet_info.columns + col;

                                let button_title = "Frame ".to_owned() + &*frame_index.to_string();
                                if ui.button(button_title).clicked() {
                                    selected_sprite_sheet.frame_index = Some(frame_index)
                                }
                            }
                        });
                    }
                }
            }
        });
    });
}