use bevy::app::{App, Plugin};
use bevy::prelude::{Res, ResMut, Resource, Update};
use bevy::utils::petgraph::visit::Walker;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use crate::editor::editor_sprite_sheet::SpriteSheets;

#[derive(Resource, Default)]
pub struct SelectedSpriteSheet {
    pub id: Option<String>,
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

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .resizable(true).show(ctx, |ui| {
        for (id, sprite_sheet_atlas) in sprite_sheets.sheets.iter() {
            if (ui.button(id)).clicked() {
                selected_sprite_sheet.id = Some(id.clone());
            }
        }
    });
}