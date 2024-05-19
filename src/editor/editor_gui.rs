use bevy::app::{App, Plugin};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::core::*;
use crate::core::core_core::{GameMode, GameState};
use crate::editor::editor_core::*;
use crate::editor::inspector::inspector_core::*;

#[derive(Default, Resource)]
pub struct EditorGuiSpace {
    pub left: f32,
    pub right: f32,
}

pub struct EditorGuiPlugin;

impl Plugin for EditorGuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InspectorPlugin)
            .add_plugins(EguiPlugin)
            .insert_resource(EditorGuiSpace::default())
            .add_systems(Update, draw_selected_frame_details);
    }
}

fn draw_selected_frame_details(
    mut egui_contexts: EguiContexts,
    mut editor_space: ResMut<EditorGuiSpace>,
    editor_sprite_sheets: ResMut<EditorSpriteSheets>,
    mut selected_frame: ResMut<SelectedFrame>,
    game_state: Res<GameState>) {
    if game_state.mode != GameMode::Editor {
        return;
    }
    let ctx = egui_contexts.ctx_mut();

    editor_space.right = egui::SidePanel::right("Selected Frame")
        .resizable(true)
        .default_width(editor_space.right)
        .show(ctx, |ui| {
            if let Some(frame_index) = selected_frame.frame_index {
                if let Some(sheet_info) = &mut selected_frame.sheet_info {
                    if let Some(frame_data) = sheet_info.frames.get_mut(frame_index) {
                        ui.collapsing("Hit Boxes", |ui| {
                            for hit_box in frame_data.hit_boxes.iter_mut() {
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
                            for hurt_box in frame_data.hurt_boxes.iter_mut() {
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

                        if ui.button("Save").clicked() {
                            let data_to_save = prepare_sprite_sheets_for_saving(&selected_frame, editor_sprite_sheets);
                            save_settings_to_file("assets/sprite_sheets.json", &data_to_save);
                        }
                    }
                }
            }
        })
        .response
        .rect
        .width();
}

fn prepare_sprite_sheets_for_saving(
    selected_frame: &SelectedFrame,
    mut sprite_sheets: ResMut<EditorSpriteSheets>) -> EditorSpriteSheetsData {
    if let Some(sheet_info) = &selected_frame.sheet_info {
        if let Some(atlas) = sprite_sheets.sheets.get_mut(&sheet_info.id) {
            atlas.sprite_sheet_info = sheet_info.clone();
        }
    }

    let sheets: Vec<EditorSpriteSheetInfo> =
        sprite_sheets.sheets.values()
            .map(|atlas|
                atlas.sprite_sheet_info.clone())
            .collect();

    EditorSpriteSheetsData { sheets }
}
