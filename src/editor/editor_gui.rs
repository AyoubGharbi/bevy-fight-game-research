use bevy::app::{App, Plugin};
use bevy::prelude::{Res, ResMut, Resource, Update};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_egui::egui::emath;

use crate::editor::editor_sprite_sheet::SpriteSheets;

#[derive(Resource)]
pub struct SelectedSpriteSheet {
    pub id: Option<String>,
    pub frame_index: Option<usize>,
}

impl Default for SelectedSpriteSheet {
    fn default() -> Self {
        SelectedSpriteSheet {
            id: None,
            frame_index: Some(0),
        }
    }
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
                let button = egui::Button::new(id)
                    .selected(Some(id.to_owned()) == selected_sprite_sheet.id);
                if ui.add(button).clicked() {
                    selected_sprite_sheet.id = Some(id.clone());
                    selected_sprite_sheet.frame_index = Some(0);
                }
            }
        });

        ui.collapsing("Selected Sprite Sheet", |ui| {
            if let Some(id) = &selected_sprite_sheet.id {
                if let Some(sprite_sheet_atlas) = sprite_sheets.sheets.get(id) {
                    let scale = 32.0 / sprite_sheet_atlas.sprite_sheet_info.tile_width as f32;
                    let scaled_width = scale * sprite_sheet_atlas.sprite_sheet_info.tile_width as f32;
                    let scaled_height = scale * sprite_sheet_atlas.sprite_sheet_info.sprite_sheet_height as f32;

                    for row in 0..sprite_sheet_atlas.sprite_sheet_info.rows {
                        ui.horizontal(|ui| {
                            for col in 0..sprite_sheet_atlas.sprite_sheet_info.columns {
                                let frame_index = row * sprite_sheet_atlas.sprite_sheet_info.columns + col;

                                let button_text = "Frame ".to_owned() + &*frame_index.to_string();
                                let button = egui::Button::new(button_text)
                                    .selected(Some(frame_index) == selected_sprite_sheet.frame_index)
                                    .min_size(emath::vec2(scaled_width, scaled_height));

                                if ui.add(button).clicked() {
                                    selected_sprite_sheet.frame_index = Some(frame_index)
                                }
                            }
                        });
                    }

                    for frame_data in &sprite_sheet_atlas.sprite_sheet_info.frames {
                        ui.collapsing("Hit Boxes", |ui| {
                            for mut hit_box in frame_data.hit_boxes.clone() {
                                ui.horizontal(|ui| {
                                    ui.label("Size");
                                    ui.add(egui::DragValue::new(&mut hit_box.size.x));
                                    ui.add(egui::DragValue::new(&mut hit_box.size.y));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Offset");
                                    ui.add(egui::DragValue::new(&mut hit_box.offset.x));
                                    ui.add(egui::DragValue::new(&mut hit_box.offset.y));
                                });
                            }
                        });

                        ui.collapsing("Hurt Boxes", |ui| {
                            for mut hurt_box in frame_data.hit_boxes.clone() {
                                ui.horizontal(|ui| {
                                    ui.label("Size");
                                    ui.add(egui::DragValue::new(&mut hurt_box.size.x));
                                    ui.add(egui::DragValue::new(&mut hurt_box.size.y));
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Offset");
                                    ui.add(egui::DragValue::new(&mut hurt_box.offset.x));
                                    ui.add(egui::DragValue::new(&mut hurt_box.offset.y));
                                });
                            }
                        });
                    }
                }
            }
        });
    });
}