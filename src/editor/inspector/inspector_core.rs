use crate::core::core_core::{GameMode, GameState};
use crate::editor::inspector::*;

pub(crate) struct InspectorPlugin;

impl Plugin for InspectorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedFrame::default())
            .add_systems(Update, inspector_setup);
    }
}

#[derive(Default, Resource)]
pub struct SelectedFrame {
    pub sprite_sheet_id: Option<String>,
    pub frame_index: Option<usize>,
    pub sheet_info: Option<EditorSpriteSheetInfo>,
}

fn inspector_setup(
    mut egui_contexts: EguiContexts,
    mut editor_space: ResMut<EditorGuiSpace>,
    mut selected_frame: ResMut<SelectedFrame>,
    editor_sprite_sheets: Res<EditorSpriteSheets>,
    game_state: Res<GameState>
) {
    if game_state.mode != GameMode::Editor {
        return;
    }
    let ctx = egui_contexts.ctx_mut();

    editor_space.left = egui::SidePanel::left("Inspector")
        .resizable(true)
        .default_width(editor_space.left)
        .show(ctx, |ui| {
            ui.collapsing("Sprite Sheets", |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for (_sheet_name, sheet_atlas) in editor_sprite_sheets.sheets.iter() {
                        let sheet_info = &sheet_atlas.sprite_sheet_info;
                        let sheet_info_label = format!("{}", sheet_info.id);
                        ui.collapsing(sheet_info_label, |ui| {
                            ui.collapsing("Frames", |ui| {
                                for (frame_index, _frame) in sheet_info.frames.iter().enumerate() {
                                    let frame_label = format!("Frame: {}", frame_index);

                                    let is_selected = selected_frame.sprite_sheet_id.as_ref() == Some(&sheet_info.id)
                                        && selected_frame.frame_index == Some(frame_index);

                                    if ui.selectable_label(is_selected, frame_label).clicked() {
                                        selected_frame.sprite_sheet_id = Some(sheet_info.id.clone());
                                        selected_frame.frame_index = Some(frame_index);
                                        selected_frame.sheet_info = Some(sheet_info.clone());
                                    }
                                }
                            });
                        });
                    }
                });
            });
        })
        .response
        .rect.width();
}